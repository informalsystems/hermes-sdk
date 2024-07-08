use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use std::collections::HashMap;

use cgp_core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use eyre::eyre;
use futures::lock::Mutex;
use hermes_error::types::Error;
use hermes_relayer_components::build::traits::birelay::ProvideBiRelayType;
use hermes_relayer_components::build::traits::cache::{HasChainCache, HasRelayCache};
use hermes_relayer_components::build::traits::components::birelay_from_relay_builder::BiRelayFromRelayBuilder;
use hermes_relayer_components::build::traits::components::chain_builder::ChainBuilder;
use hermes_relayer_components::build::traits::target::chain::{ChainATarget, ChainBTarget};
use hermes_relayer_components::build::traits::target::relay::{RelayAToBTarget, RelayBToATarget};
use hermes_relayer_components_extra::batch::traits::config::HasBatchConfig;
use hermes_relayer_components_extra::batch::types::config::BatchConfig;
use hermes_relayer_components_extra::build::traits::cache::HasBatchSenderCache;
use hermes_relayer_components_extra::build::traits::components::relay_with_batch_builder::RelayWithBatchBuilder;
use hermes_relayer_components_extra::components::extra::build::*;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};
use ibc_relayer::chain::cosmos::types::config::TxConfig;
use ibc_relayer::chain::handle::{BaseChainHandle, ChainHandle};
use ibc_relayer::config::filter::PacketFilter;
use ibc_relayer::config::{ChainConfig, Config};
use ibc_relayer::keyring::{AnySigningKeyPair, Secp256k1KeyPair};
use ibc_relayer::spawn::{spawn_chain_runtime_with_config, SpawnError};
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ClientId};
use tendermint_rpc::client::CompatMode;
use tendermint_rpc::{Client, HttpClient};
use tokio::task;

use crate::contexts::birelay::CosmosBiRelay;
use crate::contexts::chain::CosmosChain;
use crate::contexts::relay::CosmosRelay;
use crate::impls::error::HandleCosmosError;
use crate::types::batch::CosmosBatchSender;
use crate::types::telemetry::CosmosTelemetry;

#[derive(HasField)]
pub struct CosmosBuilder {
    pub config: Config,
    pub packet_filter: PacketFilter,
    pub telemetry: CosmosTelemetry,
    pub runtime: HermesRuntime,
    pub batch_config: BatchConfig,
    pub key_map: HashMap<ChainId, Secp256k1KeyPair>,
    pub chain_cache: Arc<Mutex<BTreeMap<ChainId, CosmosChain>>>,
    pub relay_cache: Arc<Mutex<BTreeMap<(ChainId, ChainId, ClientId, ClientId), CosmosRelay>>>,
    pub batch_senders:
        Arc<Mutex<BTreeMap<(ChainId, ChainId, ClientId, ClientId), CosmosBatchSender>>>,
}

pub struct CosmosBuildComponents;

pub struct CosmosBaseBuildComponents;

impl HasComponents for CosmosBuilder {
    type Components = CosmosBuildComponents;
}

impl HasComponents for CosmosBuildComponents {
    type Components = CosmosBaseBuildComponents;
}

with_extra_build_components! {
    delegate_components! {
        CosmosBuildComponents {
            @ExtraBuildComponents: ExtraBuildComponents<CosmosBaseBuildComponents>
        }
    }
}

impl CanUseExtraBuildComponents for CosmosBuilder {}

delegate_components! {
    CosmosBuildComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
        ]:
            HandleCosmosError,
        [
            RuntimeTypeComponent,
            RuntimeGetterComponent,
        ]:
            ProvideDefaultRuntimeField,
    }
}

impl ProvideBiRelayType<CosmosBuilder> for CosmosBuildComponents {
    type BiRelay = CosmosBiRelay;
}

impl CosmosBuilder {
    pub fn new_with_default(runtime: HermesRuntime) -> Self {
        Self::new(
            Default::default(),
            runtime,
            Default::default(),
            Default::default(),
            Default::default(),
            Default::default(),
        )
    }

    pub fn new(
        config: Config,
        runtime: HermesRuntime,
        telemetry: CosmosTelemetry,
        packet_filter: PacketFilter,
        batch_config: BatchConfig,
        key_map: HashMap<ChainId, Secp256k1KeyPair>,
    ) -> Self {
        Self {
            config,
            packet_filter,
            telemetry,
            runtime,
            batch_config,
            key_map,
            chain_cache: Default::default(),
            relay_cache: Default::default(),
            batch_senders: Default::default(),
        }
    }

    pub async fn build_chain(&self, chain_id: &ChainId) -> Result<CosmosChain, Error> {
        let chain_config = self
            .config
            .find_chain(chain_id)
            .cloned()
            .ok_or_else(|| SpawnError::missing_chain_config(chain_id.clone()))?;

        self.build_chain_with_config(chain_config, self.key_map.get(chain_id))
            .await
    }

    pub async fn build_chain_with_config(
        &self,
        chain_config: ChainConfig,
        m_keypair: Option<&Secp256k1KeyPair>,
    ) -> Result<CosmosChain, Error> {
        let ChainConfig::CosmosSdk(sdk_chain_config) = chain_config.clone();

        let runtime = self.runtime.runtime.clone();
        let chain_id = sdk_chain_config.id.clone();

        let (handle, key) = task::block_in_place(move || -> Result<_, Error> {
            let handle = spawn_chain_runtime_with_config::<BaseChainHandle>(chain_config, runtime)?;

            let key = get_keypair(&chain_id, &handle, m_keypair)?;

            Ok((handle, key))
        })?;

        let event_source_mode = sdk_chain_config.event_source.clone();

        let tx_config = TxConfig::try_from(&sdk_chain_config)?;

        let mut rpc_client = HttpClient::new(tx_config.rpc_address.clone())?;

        let compat_mode = if let Some(compat_mode) = &sdk_chain_config.compat_mode {
            compat_mode.clone().into()
        } else {
            let status = rpc_client.status().await?;

            CompatMode::from_version(status.node_info.version)?
        };

        rpc_client.set_compat_mode(compat_mode);

        let context = CosmosChain::new(
            handle,
            sdk_chain_config,
            tx_config,
            rpc_client,
            compat_mode,
            key,
            event_source_mode,
            self.runtime.clone(),
            self.telemetry.clone(),
        );

        Ok(context)
    }

    pub fn build_cosmos_relay(
        &self,
        src_client_id: &ClientId,
        dst_client_id: &ClientId,
        src_chain: CosmosChain,
        dst_chain: CosmosChain,
        src_batch_sender: CosmosBatchSender,
        dst_batch_sender: CosmosBatchSender,
    ) -> Result<CosmosRelay, Error> {
        let relay = CosmosRelay::new(
            self.runtime.clone(),
            src_chain,
            dst_chain,
            src_client_id.clone(),
            dst_client_id.clone(),
            self.packet_filter.clone(),
            src_batch_sender,
            dst_batch_sender,
        );

        Ok(relay)
    }
}

pub fn get_keypair(
    chain_id: &ChainId,
    handle: &BaseChainHandle,
    m_keypair: Option<&Secp256k1KeyPair>,
) -> Result<Secp256k1KeyPair, Error> {
    if let Some(keypair) = m_keypair {
        let ChainConfig::CosmosSdk(chain_config) = handle.config()?;

        // try add the key to the chain handle, in case if it is only in the key map,
        // as for the case of integration tests.
        let _ = handle.add_key(
            chain_config.key_name,
            AnySigningKeyPair::Secp256k1(keypair.clone()),
        );

        return Ok(keypair.clone());
    }

    let keypair = handle.get_key()?;

    let AnySigningKeyPair::Secp256k1(key) = keypair else {
        return Err(eyre!("no Secp256k1 key pair for chain {}", chain_id).into());
    };

    Ok(key)
}

impl ChainBuilder<CosmosBuilder, ChainATarget> for CosmosBaseBuildComponents {
    async fn build_chain(
        build: &CosmosBuilder,
        _target: ChainATarget,
        chain_id: &ChainId,
    ) -> Result<CosmosChain, Error> {
        let chain = build.build_chain(chain_id).await?;

        Ok(chain)
    }
}

impl ChainBuilder<CosmosBuilder, ChainBTarget> for CosmosBaseBuildComponents {
    async fn build_chain(
        build: &CosmosBuilder,
        _target: ChainBTarget,
        chain_id: &ChainId,
    ) -> Result<CosmosChain, Error> {
        let chain = build.build_chain(chain_id).await?;

        Ok(chain)
    }
}

impl RelayWithBatchBuilder<CosmosBuilder, RelayAToBTarget> for CosmosBuildComponents {
    async fn build_relay_with_batch(
        build: &CosmosBuilder,
        src_client_id: &ClientId,
        dst_client_id: &ClientId,
        src_chain: CosmosChain,
        dst_chain: CosmosChain,
        src_batch_sender: CosmosBatchSender,
        dst_batch_sender: CosmosBatchSender,
    ) -> Result<CosmosRelay, Error> {
        let relay = build.build_cosmos_relay(
            src_client_id,
            dst_client_id,
            src_chain,
            dst_chain,
            src_batch_sender,
            dst_batch_sender,
        )?;

        Ok(relay)
    }
}

impl RelayWithBatchBuilder<CosmosBuilder, RelayBToATarget> for CosmosBuildComponents {
    async fn build_relay_with_batch(
        build: &CosmosBuilder,
        src_client_id: &ClientId,
        dst_client_id: &ClientId,
        src_chain: CosmosChain,
        dst_chain: CosmosChain,
        src_batch_sender: CosmosBatchSender,
        dst_batch_sender: CosmosBatchSender,
    ) -> Result<CosmosRelay, Error> {
        let relay = build.build_cosmos_relay(
            src_client_id,
            dst_client_id,
            src_chain,
            dst_chain,
            src_batch_sender,
            dst_batch_sender,
        )?;

        Ok(relay)
    }
}

impl BiRelayFromRelayBuilder<CosmosBuilder> for CosmosBuildComponents {
    async fn build_birelay_from_relays(
        build: &CosmosBuilder,
        relay_a_to_b: CosmosRelay,
        relay_b_to_a: CosmosRelay,
    ) -> Result<CosmosBiRelay, Error> {
        let birelay = CosmosBiRelay {
            runtime: build.runtime.clone(),
            relay_a_to_b,
            relay_b_to_a,
        };

        Ok(birelay)
    }
}

impl HasChainCache<ChainATarget> for CosmosBuilder {
    fn chain_cache(&self) -> &Mutex<BTreeMap<ChainId, CosmosChain>> {
        &self.chain_cache
    }
}

impl HasChainCache<ChainBTarget> for CosmosBuilder {
    fn chain_cache(&self) -> &Mutex<BTreeMap<ChainId, CosmosChain>> {
        &self.chain_cache
    }
}

impl HasRelayCache<RelayAToBTarget> for CosmosBuilder {
    fn relay_cache(&self) -> &Mutex<BTreeMap<(ChainId, ChainId, ClientId, ClientId), CosmosRelay>> {
        &self.relay_cache
    }
}

impl HasRelayCache<RelayBToATarget> for CosmosBuilder {
    fn relay_cache(&self) -> &Mutex<BTreeMap<(ChainId, ChainId, ClientId, ClientId), CosmosRelay>> {
        &self.relay_cache
    }
}

impl HasBatchSenderCache<ChainATarget, Error> for CosmosBuilder {
    fn batch_sender_cache(
        &self,
        _target: ChainATarget,
    ) -> &Mutex<BTreeMap<(ChainId, ChainId, ClientId, ClientId), CosmosBatchSender>> {
        &self.batch_senders
    }
}

impl HasBatchSenderCache<ChainBTarget, Error> for CosmosBuilder {
    fn batch_sender_cache(
        &self,
        _target: ChainBTarget,
    ) -> &Mutex<BTreeMap<(ChainId, ChainId, ClientId, ClientId), CosmosBatchSender>> {
        &self.batch_senders
    }
}

impl HasBatchConfig for CosmosBuilder {
    fn batch_config(&self) -> &BatchConfig {
        &self.batch_config
    }
}
