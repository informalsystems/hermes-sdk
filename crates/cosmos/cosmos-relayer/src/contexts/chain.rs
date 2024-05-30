use alloc::sync::Arc;
use core::ops::Deref;

use futures::lock::Mutex;
use hermes_async_runtime_components::subscription::impls::empty::EmptySubscription;
use hermes_async_runtime_components::subscription::traits::subscription::Subscription;
use hermes_runtime::types::runtime::HermesRuntime;
use ibc_relayer::chain::cosmos::config::CosmosSdkConfig;
use ibc_relayer::chain::cosmos::types::config::TxConfig;
use ibc_relayer::chain::handle::BaseChainHandle;
use ibc_relayer::config::EventSourceMode;
use ibc_relayer::event::source::queries::all as all_queries;
use ibc_relayer::keyring::Secp256k1KeyPair;
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use tendermint::abci::Event as AbciEvent;
use tendermint_rpc::client::CompatMode;
use tendermint_rpc::HttpClient;

use crate::impls::subscription::CanCreateAbciEventSubscription;
use crate::types::telemetry::CosmosTelemetry;

#[derive(Clone)]
pub struct CosmosChain {
    pub base_chain: Arc<BaseCosmosChain>,
}

impl Deref for CosmosChain {
    type Target = BaseCosmosChain;

    fn deref(&self) -> &BaseCosmosChain {
        &self.base_chain
    }
}

pub struct BaseCosmosChain {
    pub handle: BaseChainHandle,
    pub chain_config: CosmosSdkConfig,
    pub chain_id: ChainId,
    pub compat_mode: CompatMode,
    pub runtime: HermesRuntime,
    pub telemetry: CosmosTelemetry,
    pub subscription: Arc<dyn Subscription<Item = (Height, Arc<AbciEvent>)>>,
    pub tx_config: TxConfig,
    pub ibc_commitment_prefix: Vec<u8>,
    pub rpc_client: HttpClient,
    pub key_entry: Secp256k1KeyPair,
    pub nonce_mutex: Mutex<()>,
}

impl CosmosChain {
    pub fn new(
        handle: BaseChainHandle,
        chain_config: CosmosSdkConfig,
        tx_config: TxConfig,
        rpc_client: HttpClient,
        compat_mode: CompatMode,
        key_entry: Secp256k1KeyPair,
        event_source_mode: EventSourceMode,
        runtime: HermesRuntime,
        telemetry: CosmosTelemetry,
    ) -> Self {
        let chain_version = tx_config.chain_id.version();

        let subscription = match event_source_mode {
            EventSourceMode::Push {
                url,
                batch_delay: _,
            } => {
                runtime.new_abci_event_subscription(chain_version, url, compat_mode, all_queries())
            }
            EventSourceMode::Pull { .. } => {
                // TODO: implement pull-based event source
                Arc::new(EmptySubscription::new())
            }
        };

        let chain_id = tx_config.chain_id.clone();
        let ibc_commitment_prefix = chain_config.store_prefix.clone().into();

        let chain = Self {
            base_chain: Arc::new(BaseCosmosChain {
                handle,
                chain_config,
                chain_id,
                compat_mode,
                runtime,
                telemetry,
                subscription,
                tx_config,
                ibc_commitment_prefix,
                rpc_client,
                key_entry,
                nonce_mutex: Mutex::new(()),
            }),
        };

        chain
    }
}
