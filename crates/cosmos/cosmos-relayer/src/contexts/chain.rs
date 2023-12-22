use alloc::sync::Arc;

use hermes_async_runtime_components::subscription::impls::empty::EmptySubscription;
use hermes_async_runtime_components::subscription::traits::subscription::Subscription;
use hermes_relayer_runtime::types::runtime::HermesRuntime;
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

use crate::contexts::transaction::CosmosTxContext;
use crate::impls::subscription::CanCreateAbciEventSubscription;
use crate::types::telemetry::CosmosTelemetry;

#[derive(Clone)]
pub struct CosmosChain {
    pub handle: BaseChainHandle,
    pub chain_id: ChainId,
    pub compat_mode: CompatMode,
    pub runtime: HermesRuntime,
    pub telemetry: CosmosTelemetry,
    pub subscription: Arc<dyn Subscription<Item = (Height, Arc<AbciEvent>)>>,
    pub tx_context: Arc<CosmosTxContext>,
}

impl CosmosChain {
    pub fn new(
        handle: BaseChainHandle,
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
            EventSourceMode::Pull { interval: _ } => {
                // TODO: implement pull-based event source
                Arc::new(EmptySubscription::new())
            }
        };

        let chain_id = tx_config.chain_id.clone();

        let tx_context = Arc::new(CosmosTxContext::new(
            tx_config,
            rpc_client,
            key_entry,
            runtime.clone(),
        ));

        let chain = Self {
            handle,
            chain_id,
            compat_mode,
            runtime,
            telemetry,
            subscription,
            tx_context,
        };

        chain
    }
}
