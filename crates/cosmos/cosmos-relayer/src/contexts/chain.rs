use alloc::sync::Arc;
use core::ops::Deref;
use hermes_logger::{HermesLogger, ProvideHermesLogger};
use hermes_logging_components::traits::logger::CanLog;
use hermes_relayer_components::transaction::impls::estimate_fees_and_send_tx::LogSendMessagesWithSignerAndNonce;
use hermes_relayer_components::transaction::impls::poll_tx_response::TxNoResponseError;

use alloc::borrow::Cow;
use cgp_core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp_core::prelude::*;
use futures::lock::Mutex;
use hermes_async_runtime_components::subscription::impls::empty::EmptySubscription;
use hermes_async_runtime_components::subscription::traits::subscription::Subscription;
use hermes_cli_components::any_client::contexts::any_counterparty::AnyCounterparty;
use hermes_cosmos_chain_components::components::client::*;
use hermes_cosmos_chain_components::components::cosmos_to_cosmos::CosmosToCosmosComponents;
use hermes_cosmos_chain_components::components::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_chain_components::components::transaction::*;
use hermes_cosmos_chain_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_chain_components::traits::gas_config::GasConfigGetter;
use hermes_cosmos_chain_components::traits::grpc_address::GrpcAddressGetter;
use hermes_cosmos_chain_components::traits::rpc_client::RpcClientGetter;
use hermes_cosmos_chain_components::traits::tx_extension_options::TxExtensionOptionsGetter;
use hermes_cosmos_chain_components::types::commitment_proof::CosmosCommitmentProof;
use hermes_cosmos_chain_components::types::nonce_guard::NonceGuard;
use hermes_cosmos_chain_components::types::tendermint::TendermintClientState;
use hermes_cosmos_chain_components::with_cosmos_tx_components;
use hermes_cosmos_test_components::chain::components::*;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetterComponent, EncodingGetterComponent, EncodingTypeComponent,
};
use hermes_error::types::Error;
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::chain::traits::commitment_prefix::IbcCommitmentPrefixGetter;
use hermes_relayer_components::chain::traits::event_subscription::HasEventSubscription;
use hermes_relayer_components::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use hermes_relayer_components::chain::traits::queries::channel_end::{
    CanQueryChannelEnd, CanQueryChannelEndWithProofs,
};
use hermes_relayer_components::chain::traits::queries::client_state::{
    CanQueryAllClientStates, CanQueryClientState, CanQueryClientStateWithProofs,
};
use hermes_relayer_components::chain::traits::queries::connection_end::{
    CanQueryConnectionEnd, CanQueryConnectionEndWithProofs,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    CanQueryConsensusState, CanQueryConsensusStateWithProofs, CanQueryRawConsensusState,
};
use hermes_relayer_components::chain::traits::queries::packet_acknowledgement::CanQueryPacketAcknowledgement;
use hermes_relayer_components::chain::traits::queries::packet_commitment::CanQueryPacketCommitment;
use hermes_relayer_components::chain::traits::queries::packet_receipt::CanQueryPacketReceipt;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdGetter;
use hermes_relayer_components::chain::traits::types::channel::HasChannelEndType;
use hermes_relayer_components::chain::traits::types::client_state::{
    HasClientStateType, HasRawClientStateType,
};
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateFields;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use hermes_relayer_components::chain::traits::types::timestamp::HasTimestampType;
use hermes_relayer_components::error::traits::retry::RetryableErrorComponent;
use hermes_relayer_components::transaction::traits::default_signer::DefaultSignerGetter;
use hermes_relayer_components::transaction::traits::nonce::nonce_mutex::ProvideMutexForNonceAllocation;
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use hermes_relayer_components::transaction::traits::query_tx_response::CanQueryTxResponse;
use hermes_relayer_components::transaction::traits::simulation_fee::FeeForSimulationGetter;
use hermes_relayer_components::transaction::traits::submit_tx::CanSubmitTx;
use hermes_relayer_components_extra::telemetry::traits::telemetry::HasTelemetry;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::mutex::MutexGuardOf;
use hermes_runtime_components::traits::runtime::RuntimeGetter;
use hermes_runtime_components::traits::runtime::RuntimeTypeComponent;
use hermes_test_components::chain::traits::assert::eventual_amount::CanAssertEventualAmount;
use hermes_test_components::chain::traits::messages::ibc_transfer::CanBuildIbcTokenTransferMessage;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::transfer::ibc_transfer::CanIbcTransferToken;
use http::Uri;
use ibc::core::channel::types::channel::ChannelEnd;
use ibc_proto::cosmos::tx::v1beta1::Fee;
use ibc_relayer::chain::cosmos::config::CosmosSdkConfig;
use ibc_relayer::chain::cosmos::types::account::Account;
use ibc_relayer::chain::cosmos::types::config::TxConfig;
use ibc_relayer::chain::cosmos::types::gas::GasConfig;
use ibc_relayer::chain::handle::BaseChainHandle;
use ibc_relayer::config::EventSourceMode;
use ibc_relayer::event::source::queries::all as all_queries;
use ibc_relayer::keyring::Secp256k1KeyPair;
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use prost_types::Any;
use tendermint::abci::Event as AbciEvent;
use tendermint_rpc::client::CompatMode;
use tendermint_rpc::{HttpClient, Url};

use crate::contexts::encoding::ProvideCosmosEncoding;
use crate::impls::error::HandleCosmosError;
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

pub struct CosmosChainComponents;

impl HasComponents for CosmosChain {
    type Components = CosmosChainComponents;
}

delegate_components! {
    CosmosChainComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
            RetryableErrorComponent,
        ]:
            HandleCosmosError,
        RuntimeTypeComponent:
            ProvideHermesRuntime,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideHermesLogger,
        [
            EncodingTypeComponent,
            EncodingGetterComponent,
            DefaultEncodingGetterComponent,
        ]:
            ProvideCosmosEncoding,
    }
}

with_cosmos_client_components! {
    delegate_components! {
        CosmosChainComponents {
            @CosmosClientComponents: CosmosClientComponents,
        }
    }
}

with_cosmos_tx_components! {
    delegate_components! {
        CosmosChainComponents {
            @CosmosTxComponents : CosmosTxComponents,
        }
    }
}

with_cosmmos_chain_test_components! {
    delegate_components! {
        CosmosChainComponents {
            @CosmmosChainTestComponents: CosmmosChainTestComponents,
        }
    }
}

delegate_components! {
    DelegateCosmosChainComponents {
        CosmosChain: CosmosToCosmosComponents,
    }
}

impl TxExtensionOptionsGetter<CosmosChain> for CosmosChainComponents {
    fn tx_extension_options(chain: &CosmosChain) -> &Vec<ibc_proto::google::protobuf::Any> {
        &chain.tx_config.extension_options
    }
}

impl GasConfigGetter<CosmosChain> for CosmosChainComponents {
    fn gas_config(chain: &CosmosChain) -> &GasConfig {
        &chain.tx_config.gas_config
    }
}

impl DefaultSignerGetter<CosmosChain> for CosmosChainComponents {
    fn get_default_signer(chain: &CosmosChain) -> &Secp256k1KeyPair {
        &chain.key_entry
    }
}

impl FeeForSimulationGetter<CosmosChain> for CosmosChainComponents {
    fn fee_for_simulation(chain: &CosmosChain) -> &Fee {
        &chain.tx_config.gas_config.max_fee
    }
}

impl ProvideMutexForNonceAllocation<CosmosChain> for CosmosChainComponents {
    fn mutex_for_nonce_allocation<'a>(
        chain: &'a CosmosChain,
        _signer: &Secp256k1KeyPair,
    ) -> &'a Mutex<()> {
        &chain.nonce_mutex
    }

    fn mutex_to_nonce_guard<'a>(
        mutex_guard: MutexGuardOf<'a, HermesRuntime, ()>,
        account: Account,
    ) -> NonceGuard<'a> {
        NonceGuard {
            mutex_guard,
            account,
        }
    }
}

impl IbcCommitmentPrefixGetter<CosmosChain> for CosmosChainComponents {
    fn ibc_commitment_prefix(chain: &CosmosChain) -> &Vec<u8> {
        &chain.ibc_commitment_prefix
    }
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

impl RuntimeGetter<CosmosChain> for CosmosChainComponents {
    fn runtime(chain: &CosmosChain) -> &HermesRuntime {
        &chain.runtime
    }
}

impl HasTelemetry for CosmosChain {
    type Telemetry = CosmosTelemetry;

    fn telemetry(&self) -> &CosmosTelemetry {
        &self.telemetry
    }
}

impl GrpcAddressGetter<CosmosChain> for CosmosChainComponents {
    fn grpc_address(chain: &CosmosChain) -> &Uri {
        &chain.tx_config.grpc_address
    }
}

impl RpcClientGetter<CosmosChain> for CosmosChainComponents {
    fn rpc_client(chain: &CosmosChain) -> &HttpClient {
        &chain.rpc_client
    }

    fn rpc_address(chain: &CosmosChain) -> &Url {
        &chain.tx_config.rpc_address
    }
}

impl HasBlockingChainHandle for CosmosChain {
    type ChainHandle = BaseChainHandle;

    async fn with_blocking_chain_handle<R>(
        &self,
        cont: impl FnOnce(BaseChainHandle) -> Result<R, Error> + Send + 'static,
    ) -> Result<R, Error>
    where
        R: Send + 'static,
    {
        let chain_handle = self.handle.clone();

        self.runtime
            .runtime
            .spawn_blocking(move || cont(chain_handle))
            .await?
    }
}

impl ChainIdGetter<CosmosChain> for CosmosChainComponents {
    fn chain_id(chain: &CosmosChain) -> &ChainId {
        &chain.chain_id
    }
}

impl HasEventSubscription for CosmosChain {
    fn event_subscription(&self) -> &Arc<dyn Subscription<Item = (Height, Arc<AbciEvent>)>> {
        &self.subscription
    }
}

impl<Counterparty> HasConsensusStateFields<Counterparty> for CosmosChain
where
    Counterparty: HasTimestampType,
{
    fn consensus_state_timestamp(
        consensus_state: &Self::ConsensusState,
    ) -> Cow<'_, Counterparty::Timestamp> {
        // FIXME(romac): This is a temporary workaround until we have a proper conversion,
        // and can blow out if the timestamp is later than July 21st, 2554.
        let nanos = consensus_state.timestamp.unix_timestamp_nanos() as u64;
        Cow::Owned(Counterparty::timestamp_from_nanos(nanos))
    }
}

pub trait CanUseCosmosChain:
    HasClientStateType<CosmosChain, ClientState = TendermintClientState>
    + HasChannelEndType<CosmosChain, ChannelEnd = ChannelEnd>
    + HasCommitmentProofType<CommitmentProof = CosmosCommitmentProof>
    + CanQueryBalance
    + CanIbcTransferToken<CosmosChain>
    + CanBuildIbcTokenTransferMessage<CosmosChain>
    + CanQueryClientState<CosmosChain>
    + CanQueryClientStateWithProofs<CosmosChain>
    + CanQueryConsensusState<CosmosChain>
    + CanQueryConsensusStateWithProofs<CosmosChain>
    + CanQueryRawConsensusState<CosmosChain>
    + CanQueryAllClientStates<CosmosChain>
    + CanQueryClientState<AnyCounterparty>
    + CanQueryAllClientStates<AnyCounterparty>
    + CanBuildUpdateClientMessage<CosmosChain>
    + CanQueryConnectionEnd<CosmosChain>
    + CanQueryChannelEnd<CosmosChain>
    + CanQueryChannelEndWithProofs<CosmosChain>
    + CanQueryConnectionEndWithProofs<CosmosChain>
    + CanQueryPacketCommitment<CosmosChain>
    + CanQueryPacketAcknowledgement<CosmosChain>
    + CanQueryPacketReceipt<CosmosChain>
    + HasRawClientStateType<RawClientState = Any>
    + CanSubmitTx
    + CanPollTxResponse
    + CanQueryTxResponse
    + CanAssertEventualAmount
{
}

impl CanUseCosmosChain for CosmosChain {}

pub trait CanUseLoggerWithCosmosChain:
    for<'a> CanLog<LogSendMessagesWithSignerAndNonce<'a, CosmosChain>>
    + for<'a> CanLog<TxNoResponseError<'a, CosmosChain>>
{
}

impl CanUseLoggerWithCosmosChain for HermesLogger {}
