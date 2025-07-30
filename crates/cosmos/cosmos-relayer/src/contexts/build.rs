use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use core::marker::PhantomData;
use core::ops::Deref;
use std::collections::HashMap;
use std::fs::File;
use std::str::FromStr;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent};
use cgp::core::field::{Index, UseField, WithField};
use cgp::core::types::WithType;
use eyre::{eyre, Report};
use futures::lock::Mutex;
use hermes_core::relayer_components::build::traits::builders::birelay_from_relay_builder::{
    BiRelayFromRelayBuilder, BiRelayFromRelayBuilderComponent,
};
use hermes_core::relayer_components::build::traits::builders::chain_builder::{
    ChainBuilder, ChainBuilderComponent,
};
use hermes_core::relayer_components::build::traits::cache::{HasChainCache, HasRelayCache};
use hermes_core::relayer_components::multi::traits::birelay_at::BiRelayTypeProviderAtComponent;
use hermes_core::relayer_components::multi::traits::chain_at::{
    ChainTypeProviderAtComponent, HasChainTypeAt,
};
use hermes_core::relayer_components::multi::traits::relay_at::{
    HasRelayTypeAt, RelayTypeProviderAtComponent,
};
use hermes_core::relayer_components::multi::types::tags::{Dst, Src};
use hermes_core::relayer_components::relay::traits::SourceTarget;
use hermes_core::relayer_components_extra::batch::traits::config::HasBatchConfig;
use hermes_core::relayer_components_extra::batch::traits::types::MessageBatchSenderOf;
use hermes_core::relayer_components_extra::batch::types::config::BatchConfig;
use hermes_core::relayer_components_extra::build::traits::cache::{
    BatchSenderCacheAt, BatchSenderCacheGetterComponent,
};
use hermes_core::relayer_components_extra::build::traits::relay_with_batch_builder::{
    RelayWithBatchBuilder, RelayWithBatchBuilderComponent,
};
use hermes_core::relayer_components_extra::components::extra::build::ExtraBuildComponents;
use hermes_core::runtime_components::traits::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_cosmos_core::chain_components::impls::CosmosChainConfig;
use hermes_cosmos_core::chain_components::types::{
    PacketFilterConfig, Secp256k1KeyPair, KEYSTORE_DEFAULT_FOLDER, KEYSTORE_FILE_EXTENSION,
};
use hermes_error::types::Error;
use hermes_prelude::*;
use hermes_runtime::types::runtime::HermesRuntime;
use ibc::core::host::types::identifiers::{ChainId, ClientId};
use tendermint_rpc::client::CompatMode;
use tendermint_rpc::{Client, HttpClient};

use crate::contexts::{CosmosBiRelay, CosmosChain, CosmosRelay};
use crate::impls::HandleCosmosError;
use crate::types::telemetry::CosmosTelemetry;

#[cgp_context(
    CosmosBuildComponents:
        ExtraBuildComponents<CosmosBaseBuildComponents>
)]
#[derive(Clone)]
pub struct CosmosBuilder {
    pub fields: Arc<dyn HasCosmosBuilderFields>,
}

#[derive(HasField)]
pub struct CosmosBuilderFields {
    pub config_map: HashMap<ChainId, CosmosChainConfig>,
    pub packet_filter: PacketFilterConfig,
    pub telemetry: CosmosTelemetry,
    pub runtime: HermesRuntime,
    pub batch_config: BatchConfig,
    pub key_map: HashMap<ChainId, Secp256k1KeyPair>,
    pub chain_cache: Arc<Mutex<BTreeMap<ChainId, CosmosChain>>>,
    pub relay_cache: Arc<Mutex<BTreeMap<(ChainId, ChainId, ClientId, ClientId), CosmosRelay>>>,
    pub batch_senders: BatchSenderCacheAt<CosmosBuilder, Index<0>, Index<1>, SourceTarget>,
}

impl Deref for CosmosBuilder {
    type Target = CosmosBuilderFields;

    fn deref(&self) -> &Self::Target {
        self.fields.fields()
    }
}

pub trait HasCosmosBuilderFields: Send + Sync + 'static {
    fn fields(&self) -> &CosmosBuilderFields;
}

impl HasCosmosBuilderFields for CosmosBuilderFields {
    fn fields(&self) -> &CosmosBuilderFields {
        self
    }
}

pub struct CosmosBaseBuildComponents;

delegate_components! {
    CosmosBuildComponents {
        [
            ErrorTypeProviderComponent,
            ErrorRaiserComponent,
        ]:
            HandleCosmosError,
        RuntimeTypeProviderComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        BiRelayTypeProviderAtComponent<Index<0>, Index<1>>:
            WithType<CosmosBiRelay>,
        [
            ChainTypeProviderAtComponent<Index<0>>,
            ChainTypeProviderAtComponent<Index<1>>,
        ]:
            WithType<CosmosChain>,
        [
            RelayTypeProviderAtComponent<Index<0>, Index<1>>,
            RelayTypeProviderAtComponent<Index<1>, Index<0>>,
        ]: WithType<CosmosRelay>,
        BatchSenderCacheGetterComponent:
            UseField<symbol!("batch_senders")>,
    }
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
        chain_configs: Vec<CosmosChainConfig>,
        runtime: HermesRuntime,
        telemetry: CosmosTelemetry,
        packet_filter: PacketFilterConfig,
        batch_config: BatchConfig,
        key_map: HashMap<ChainId, Secp256k1KeyPair>,
    ) -> Self {
        let config_map = HashMap::from_iter(
            chain_configs
                .into_iter()
                .map(|config| (ChainId::new(&config.id).unwrap(), config)),
        );

        Self {
            fields: Arc::new(CosmosBuilderFields {
                config_map,
                packet_filter,
                telemetry,
                runtime,
                batch_config,
                key_map,
                chain_cache: Default::default(),
                relay_cache: Default::default(),
                batch_senders: Default::default(),
            }),
        }
    }

    pub async fn build_chain(&self, chain_id: &ChainId) -> Result<CosmosChain, Error> {
        let chain_config = self.config_map.get(chain_id).cloned().ok_or_else(|| {
            Report::msg(format!("missing configuration for chain ID `{chain_id}`"))
        })?;

        self.build_chain_with_config(chain_config).await
    }

    pub async fn build_chain_with_config(
        &self,
        chain_config: CosmosChainConfig,
    ) -> Result<CosmosChain, Error> {
        let keys = get_keypair(&chain_config)?;

        let mut rpc_client = HttpClient::new(chain_config.rpc_addr.clone())?;

        let compat_mode = if let Some(compat_mode) = &chain_config.compat_mode {
            CompatMode::from_str(compat_mode.as_str()).unwrap()
        } else {
            let status = rpc_client.status().await?;

            CompatMode::from_version(status.node_info.version)?
        };

        rpc_client.set_compat_mode(compat_mode);

        let context = CosmosChain::new(
            chain_config,
            rpc_client,
            compat_mode,
            keys,
            self.runtime.clone(),
            self.telemetry.clone(),
            self.packet_filter.clone(),
        );

        Ok(context)
    }

    pub fn build_cosmos_relay(
        &self,
        src_client_id: &ClientId,
        dst_client_id: &ClientId,
        src_chain: CosmosChain,
        dst_chain: CosmosChain,
        src_batch_sender: MessageBatchSenderOf<CosmosRelay, Src>,
        dst_batch_sender: MessageBatchSenderOf<CosmosRelay, Dst>,
    ) -> Result<CosmosRelay, Error> {
        let relay = CosmosRelay::new(
            self.runtime.clone(),
            src_chain,
            dst_chain,
            src_client_id.clone(),
            dst_client_id.clone(),
            src_batch_sender,
            dst_batch_sender,
        );

        Ok(relay)
    }
}

pub fn get_keypair(chain_config: &CosmosChainConfig) -> Result<Vec<Secp256k1KeyPair>, Error> {
    let ks_folder = &chain_config.key_store_folder;

    let ks_folder = match ks_folder {
        Some(folder) => folder.to_owned(),
        None => {
            let home =
                dirs_next::home_dir().ok_or_else(|| eyre!("failed to retrieve home directory"))?;
            home.join(KEYSTORE_DEFAULT_FOLDER)
        }
    };

    let mut key_entries: Vec<Secp256k1KeyPair> = vec![];
    for key_name in chain_config.key_names.iter() {
        let mut filename = ks_folder.join(key_name.clone());
        filename.set_extension(KEYSTORE_FILE_EXTENSION);

        let file = File::open(&filename)?;

        let key_entry = serde_json::from_reader(file)?;
        key_entries.push(key_entry);
    }

    Ok(key_entries)
}

#[cgp_provider(ChainBuilderComponent)]
impl ChainBuilder<CosmosBuilder, Index<0>> for CosmosBaseBuildComponents {
    async fn build_chain(
        build: &CosmosBuilder,
        _index: PhantomData<Index<0>>,
        chain_id: &ChainId,
    ) -> Result<CosmosChain, Error> {
        build.build_chain(chain_id).await
    }
}

#[cgp_provider(ChainBuilderComponent)]
impl ChainBuilder<CosmosBuilder, Index<1>> for CosmosBaseBuildComponents {
    async fn build_chain(
        build: &CosmosBuilder,
        _index: PhantomData<Index<1>>,
        chain_id: &ChainId,
    ) -> Result<CosmosChain, Error> {
        build.build_chain(chain_id).await
    }
}

#[cgp_provider(RelayWithBatchBuilderComponent)]
impl RelayWithBatchBuilder<CosmosBuilder, Index<0>, Index<1>> for CosmosBuildComponents {
    async fn build_relay_with_batch(
        build: &CosmosBuilder,
        _index: PhantomData<(Index<0>, Index<1>)>,
        src_client_id: &ClientId,
        dst_client_id: &ClientId,
        src_chain: CosmosChain,
        dst_chain: CosmosChain,
        src_batch_sender: MessageBatchSenderOf<CosmosRelay, Src>,
        dst_batch_sender: MessageBatchSenderOf<CosmosRelay, Dst>,
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

#[cgp_provider(RelayWithBatchBuilderComponent)]
impl RelayWithBatchBuilder<CosmosBuilder, Index<1>, Index<0>> for CosmosBuildComponents {
    async fn build_relay_with_batch(
        build: &CosmosBuilder,
        _index: PhantomData<(Index<1>, Index<0>)>,
        src_client_id: &ClientId,
        dst_client_id: &ClientId,
        src_chain: CosmosChain,
        dst_chain: CosmosChain,
        src_batch_sender: MessageBatchSenderOf<CosmosRelay, Src>,
        dst_batch_sender: MessageBatchSenderOf<CosmosRelay, Dst>,
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

#[cgp_provider(BiRelayFromRelayBuilderComponent)]
impl BiRelayFromRelayBuilder<CosmosBuilder, Index<0>, Index<1>> for CosmosBuildComponents {
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

impl HasChainCache<Index<0>> for CosmosBuilder {
    fn chain_cache(&self) -> &Mutex<BTreeMap<ChainId, CosmosChain>> {
        &self.chain_cache
    }
}

impl HasChainCache<Index<1>> for CosmosBuilder {
    fn chain_cache(&self) -> &Mutex<BTreeMap<ChainId, CosmosChain>> {
        &self.chain_cache
    }
}

impl HasRelayCache<Index<0>, Index<1>> for CosmosBuilder {
    fn relay_cache(&self) -> &Mutex<BTreeMap<(ChainId, ChainId, ClientId, ClientId), CosmosRelay>> {
        &self.relay_cache
    }
}

impl HasRelayCache<Index<1>, Index<0>> for CosmosBuilder {
    fn relay_cache(&self) -> &Mutex<BTreeMap<(ChainId, ChainId, ClientId, ClientId), CosmosRelay>> {
        &self.relay_cache
    }
}

impl HasBatchConfig for CosmosBuilder {
    fn batch_config(&self) -> &BatchConfig {
        &self.batch_config
    }
}

pub trait CanUseCosmosBuilder:
    HasChainTypeAt<Index<0>, Chain = CosmosChain>
    + HasChainTypeAt<Index<1>, Chain = CosmosChain>
    + HasRelayTypeAt<Index<0>, Index<1>, Relay = CosmosRelay>
    + HasRelayTypeAt<Index<1>, Index<0>, Relay = CosmosRelay>
{
}

impl CanUseCosmosBuilder for CosmosBuilder {}
