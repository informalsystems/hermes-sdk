use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use eyre::eyre;
use futures::lock::Mutex;
use std::collections::HashMap;
use std::fs::{self, File};

use hermes_cosmos_chain_components::types::config::tx_config::TxConfig;
use hermes_error::types::Error;
use hermes_relayer_components::build::traits::builders::birelay_from_relay_builder::BiRelayFromRelayBuilder;
use hermes_relayer_components::build::traits::builders::chain_builder::ChainBuilder;
use hermes_relayer_components::build::traits::cache::{HasChainCache, HasRelayCache};
use hermes_relayer_components::multi::traits::birelay_at::ProvideBiRelayTypeAt;
use hermes_relayer_components::multi::traits::chain_at::ProvideChainTypeAt;
use hermes_relayer_components::multi::traits::relay_at::ProvideRelayTypeAt;
use hermes_relayer_components::multi::types::index::{Index, Twindex};
use hermes_relayer_components_extra::batch::traits::config::HasBatchConfig;
use hermes_relayer_components_extra::batch::types::config::BatchConfig;
use hermes_relayer_components_extra::build::traits::cache::HasBatchSenderCache;
use hermes_relayer_components_extra::build::traits::relay_with_batch_builder::RelayWithBatchBuilder;
use hermes_relayer_components_extra::components::extra::build::*;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};
use ibc_relayer::chain::cosmos::config::CosmosSdkConfig;
use ibc_relayer::config::filter::PacketFilter;
use ibc_relayer::keyring::{
    AnySigningKeyPair, Secp256k1KeyPair, KEYSTORE_DEFAULT_FOLDER, KEYSTORE_DISK_BACKEND,
    KEYSTORE_FILE_EXTENSION,
};
use ibc_relayer::spawn::SpawnError;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ClientId};
use tendermint_rpc::client::CompatMode;
use tendermint_rpc::{Client, HttpClient};

use crate::contexts::birelay::CosmosBiRelay;
use crate::contexts::chain::CosmosChain;
use crate::contexts::relay::CosmosRelay;
use crate::impls::error::HandleCosmosError;
use crate::types::batch::CosmosBatchSender;
use crate::types::telemetry::CosmosTelemetry;

#[derive(HasField)]
pub struct CosmosBuilder {
    pub config_map: HashMap<ChainId, CosmosSdkConfig>,
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

impl ProvideBiRelayTypeAt<CosmosBuilder, 0, 1> for CosmosBuildComponents {
    type BiRelay = CosmosBiRelay;
}

impl ProvideChainTypeAt<CosmosBuilder, 0> for CosmosBuildComponents {
    type Chain = CosmosChain;
}

impl ProvideChainTypeAt<CosmosBuilder, 1> for CosmosBuildComponents {
    type Chain = CosmosChain;
}

impl ProvideRelayTypeAt<CosmosBuilder, 0, 1> for CosmosBuildComponents {
    type Relay = CosmosRelay;
}

impl ProvideRelayTypeAt<CosmosBuilder, 1, 0> for CosmosBuildComponents {
    type Relay = CosmosRelay;
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
        chain_configs: Vec<CosmosSdkConfig>,
        runtime: HermesRuntime,
        telemetry: CosmosTelemetry,
        packet_filter: PacketFilter,
        batch_config: BatchConfig,
        key_map: HashMap<ChainId, Secp256k1KeyPair>,
    ) -> Self {
        let config_map = HashMap::from_iter(
            chain_configs
                .into_iter()
                .map(|config| (config.id.clone(), config)),
        );

        Self {
            config_map,
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
            .config_map
            .get(chain_id)
            .cloned()
            .ok_or_else(|| SpawnError::missing_chain_config(chain_id.clone()))?;

        self.build_chain_with_config(chain_config, self.key_map.get(chain_id))
            .await
    }

    pub async fn build_chain_with_config(
        &self,
        chain_config: CosmosSdkConfig,
        m_keypair: Option<&Secp256k1KeyPair>,
    ) -> Result<CosmosChain, Error> {
        let chain_id = chain_config.id.clone();

        let key = get_keypair(&chain_id, &chain_config, m_keypair)?;

        let event_source_mode = chain_config.event_source.clone();

        let tx_config = TxConfig::try_from(&chain_config)?;

        let mut rpc_client = HttpClient::new(tx_config.rpc_address.clone())?;

        let compat_mode = if let Some(compat_mode) = &chain_config.compat_mode {
            *compat_mode
        } else {
            let status = rpc_client.status().await?;

            CompatMode::from_version(status.node_info.version)?
        };

        rpc_client.set_compat_mode(compat_mode);

        let context = CosmosChain::new(
            chain_config,
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
    chain_config: &CosmosSdkConfig,
    m_keypair: Option<&Secp256k1KeyPair>,
) -> Result<Secp256k1KeyPair, Error> {
    let ks_folder = &chain_config.key_store_folder;

    let ks_folder = match ks_folder {
        Some(folder) => folder.to_owned(),
        None => {
            let home =
                dirs_next::home_dir().ok_or_else(|| eyre!("failed to retrieve home directory"))?;
            home.join(KEYSTORE_DEFAULT_FOLDER)
        }
    };

    let keys_folder = ks_folder
        .join(chain_id.as_str())
        .join(KEYSTORE_DISK_BACKEND);

    let mut filename = keys_folder.join(chain_config.key_name.clone());
    filename.set_extension(KEYSTORE_FILE_EXTENSION);

    let file = File::create(filename.clone())?;

    if let Some(keypair) = m_keypair {
        // Create keys folder if it does not exist
        fs::create_dir_all(&keys_folder)?;

        serde_json::to_writer_pretty(file, &AnySigningKeyPair::Secp256k1(keypair.clone()))?;
    }

    let file = File::open(&filename)?;

    let key_entry = serde_json::from_reader(file)?;

    Ok(key_entry)
}

impl ChainBuilder<CosmosBuilder, 0> for CosmosBaseBuildComponents {
    async fn build_chain(
        build: &CosmosBuilder,
        _index: Index<0>,
        chain_id: &ChainId,
    ) -> Result<CosmosChain, Error> {
        let chain = build.build_chain(chain_id).await?;

        Ok(chain)
    }
}

impl ChainBuilder<CosmosBuilder, 1> for CosmosBaseBuildComponents {
    async fn build_chain(
        build: &CosmosBuilder,
        _index: Index<1>,
        chain_id: &ChainId,
    ) -> Result<CosmosChain, Error> {
        let chain = build.build_chain(chain_id).await?;

        Ok(chain)
    }
}

impl RelayWithBatchBuilder<CosmosBuilder, 0, 1> for CosmosBuildComponents {
    async fn build_relay_with_batch(
        build: &CosmosBuilder,
        _index: Twindex<0, 1>,
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

impl RelayWithBatchBuilder<CosmosBuilder, 1, 0> for CosmosBuildComponents {
    async fn build_relay_with_batch(
        build: &CosmosBuilder,
        _index: Twindex<1, 0>,
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

impl BiRelayFromRelayBuilder<CosmosBuilder, 0, 1> for CosmosBuildComponents {
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

impl HasChainCache<0> for CosmosBuilder {
    fn chain_cache(&self) -> &Mutex<BTreeMap<ChainId, CosmosChain>> {
        &self.chain_cache
    }
}

impl HasChainCache<1> for CosmosBuilder {
    fn chain_cache(&self) -> &Mutex<BTreeMap<ChainId, CosmosChain>> {
        &self.chain_cache
    }
}

impl HasRelayCache<0, 1> for CosmosBuilder {
    fn relay_cache(&self) -> &Mutex<BTreeMap<(ChainId, ChainId, ClientId, ClientId), CosmosRelay>> {
        &self.relay_cache
    }
}

impl HasRelayCache<1, 0> for CosmosBuilder {
    fn relay_cache(&self) -> &Mutex<BTreeMap<(ChainId, ChainId, ClientId, ClientId), CosmosRelay>> {
        &self.relay_cache
    }
}

impl HasBatchSenderCache<Error, 0, 1> for CosmosBuilder {
    fn batch_sender_cache(
        &self,
        _index: Twindex<0, 1>,
    ) -> &Mutex<BTreeMap<(ChainId, ChainId, ClientId, ClientId), CosmosBatchSender>> {
        &self.batch_senders
    }
}

impl HasBatchSenderCache<Error, 1, 0> for CosmosBuilder {
    fn batch_sender_cache(
        &self,
        _index: Twindex<1, 0>,
    ) -> &Mutex<BTreeMap<(ChainId, ChainId, ClientId, ClientId), CosmosBatchSender>> {
        &self.batch_senders
    }
}

impl HasBatchConfig for CosmosBuilder {
    fn batch_config(&self) -> &BatchConfig {
        &self.batch_config
    }
}
