use alloc::sync::Arc;
use core::ops::Deref;
use core::str::FromStr;
use hermes_cosmos_chain_components::impls::types::config::{CosmosChainConfig, EventSourceMode};
use hermes_cosmos_chain_components::traits::convert_gas_to_fee::CanConvertGasToFee;
use hermes_cosmos_chain_components::traits::eip::eip_query::CanQueryEipBaseFee;
use hermes_cosmos_chain_components::traits::unbonding_period::CanQueryUnbondingPeriod;
use hermes_cosmos_chain_components::types::config::gas::gas_config::GasConfig;
use hermes_cosmos_chain_components::types::config::tx_config::TxConfig;
use hermes_cosmos_chain_components::types::payloads::client::{
    CosmosCreateClientOptions, CosmosCreateClientPayload, CosmosUpdateClientPayload,
};
use hermes_relayer_components::chain::traits::payload_builders::create_client::CanBuildCreateClientPayload;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientPayloadOptionsType, HasCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::prelude::*;
use futures::lock::Mutex;
use hermes_any_counterparty::contexts::any_counterparty::AnyCounterparty;
use hermes_async_runtime_components::subscription::impls::empty::EmptySubscription;
use hermes_async_runtime_components::subscription::traits::subscription::Subscription;
use hermes_chain_type_components::traits::fields::message_response_events::HasMessageResponseEvents;
use hermes_chain_type_components::traits::types::event::HasEventType;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_cosmos_chain_components::components::client::*;
use hermes_cosmos_chain_components::components::cosmos_to_cosmos::CosmosToCosmosComponents;
use hermes_cosmos_chain_components::components::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_chain_components::components::transaction::*;
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
use hermes_logger::{HermesLogger, ProvideHermesLogger};
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_logging_components::traits::logger::CanLog;
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
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::HasSendPacketEvent;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use hermes_relayer_components::error::traits::retry::RetryableErrorComponent;
use hermes_relayer_components::transaction::impls::estimate_fees_and_send_tx::LogSendMessagesWithSignerAndNonce;
use hermes_relayer_components::transaction::impls::poll_tx_response::TxNoResponseError;
use hermes_relayer_components::transaction::traits::default_signer::DefaultSignerGetter;
use hermes_relayer_components::transaction::traits::nonce::nonce_mutex::ProvideMutexForNonceAllocation;
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use hermes_relayer_components::transaction::traits::query_tx_response::CanQueryTxResponse;
use hermes_relayer_components::transaction::traits::simulation_fee::FeeForSimulationGetter;
use hermes_relayer_components::transaction::traits::submit_tx::CanSubmitTx;
use hermes_relayer_components_extra::telemetry::traits::telemetry::HasTelemetry;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::mutex::MutexGuardOf;
use hermes_runtime_components::traits::runtime::{
    ProvideDefaultRuntimeField, RuntimeGetterComponent, RuntimeTypeComponent,
};
use hermes_test_components::chain::traits::assert::eventual_amount::CanAssertEventualAmount;
use hermes_test_components::chain::traits::messages::ibc_transfer::CanBuildIbcTokenTransferMessage;
use hermes_test_components::chain::traits::proposal::messages::deposit::CanBuildDepositProposalMessage;
use hermes_test_components::chain::traits::proposal::messages::vote::CanBuildVoteProposalMessage;
use hermes_test_components::chain::traits::proposal::query_status::CanQueryProposalStatus;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;
use hermes_test_components::chain::traits::transfer::ibc_transfer::CanIbcTransferToken;
use hermes_wasm_test_components::components::WasmChainComponents;
use hermes_wasm_test_components::traits::chain::messages::store_code::StoreCodeMessageBuilderComponent;
use hermes_wasm_test_components::traits::chain::upload_client_code::{
    CanUploadWasmClientCode, WasmClientCodeUploaderComponent,
};
use http::Uri;
use ibc::core::channel::types::channel::ChannelEnd;
use ibc_proto::cosmos::tx::v1beta1::Fee;
use ibc_relayer::chain::cosmos::types::account::Account;
use ibc_relayer::event::source::queries::all as all_queries;
use ibc_relayer::keyring::Secp256k1KeyPair;
use ibc_relayer_types::core::ics02_client::height::Height;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use prost_types::Any;
use tendermint::abci::Event as AbciEvent;
use tendermint_rpc::client::CompatMode;
use tendermint_rpc::{HttpClient, Url, WebSocketClientUrl};

use crate::contexts::encoding::ProvideCosmosEncoding;
use crate::impls::error::HandleCosmosError;
use crate::impls::subscription::CanCreateAbciEventSubscription;
use crate::types::telemetry::CosmosTelemetry;

#[derive(Clone)]
pub struct CosmosChain {
    pub base_chain: Arc<BaseCosmosChain>,
}

#[derive(HasField)]
pub struct BaseCosmosChain {
    pub chain_config: CosmosChainConfig,
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

impl Deref for CosmosChain {
    type Target = BaseCosmosChain;

    fn deref(&self) -> &BaseCosmosChain {
        &self.base_chain
    }
}

pub struct CosmosChainContextComponents;

impl HasComponents for CosmosChain {
    type Components = CosmosChainContextComponents;
}

delegate_components! {
    CosmosChainContextComponents {
        [
            ErrorTypeComponent,
            ErrorRaiserComponent,
            RetryableErrorComponent,
        ]:
            HandleCosmosError,
        [
            RuntimeTypeComponent,
            RuntimeGetterComponent,
        ]:
            ProvideDefaultRuntimeField,
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
        [
            StoreCodeMessageBuilderComponent,
            WasmClientCodeUploaderComponent,
        ]:
            WasmChainComponents,
    }
}

with_cosmos_client_components! {
    delegate_components! {
        CosmosChainContextComponents {
            @CosmosClientComponents: CosmosClientComponents,
        }
    }
}

with_cosmos_tx_components! {
    delegate_components! {
        CosmosChainContextComponents {
            @CosmosTxComponents : CosmosTxComponents,
        }
    }
}

with_cosmmos_chain_test_components! {
    delegate_components! {
        CosmosChainContextComponents {
            @CosmmosChainTestComponents: CosmmosChainTestComponents,
        }
    }
}

delegate_components! {
    DelegateCosmosChainComponents {
        CosmosChain: CosmosToCosmosComponents,
    }
}

impl TxExtensionOptionsGetter<CosmosChain> for CosmosChainContextComponents {
    fn tx_extension_options(chain: &CosmosChain) -> &Vec<ibc_proto::google::protobuf::Any> {
        &chain.tx_config.extension_options
    }
}

impl GasConfigGetter<CosmosChain> for CosmosChainContextComponents {
    fn gas_config(chain: &CosmosChain) -> &GasConfig {
        &chain.tx_config.gas_config
    }
}

impl DefaultSignerGetter<CosmosChain> for CosmosChainContextComponents {
    fn get_default_signer(chain: &CosmosChain) -> &Secp256k1KeyPair {
        &chain.key_entry
    }
}

impl FeeForSimulationGetter<CosmosChain> for CosmosChainContextComponents {
    fn fee_for_simulation(chain: &CosmosChain) -> &Fee {
        &chain.tx_config.gas_config.max_fee
    }
}

impl ProvideMutexForNonceAllocation<CosmosChain> for CosmosChainContextComponents {
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

impl IbcCommitmentPrefixGetter<CosmosChain> for CosmosChainContextComponents {
    fn ibc_commitment_prefix(chain: &CosmosChain) -> &Vec<u8> {
        &chain.ibc_commitment_prefix
    }
}

impl CosmosChain {
    pub fn new(
        chain_config: CosmosChainConfig,
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
            EventSourceMode::Push { url } => runtime.new_abci_event_subscription(
                chain_version,
                WebSocketClientUrl::from_str(&url).unwrap(),
                compat_mode,
                all_queries(),
            ),
            EventSourceMode::Pull { .. } => {
                // TODO: implement pull-based event source
                Arc::new(EmptySubscription::new())
            }
        };

        let chain_id = tx_config.chain_id.clone();
        let ibc_commitment_prefix = chain_config.store_prefix.clone().into();

        let chain = Self {
            base_chain: Arc::new(BaseCosmosChain {
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

impl HasTelemetry for CosmosChain {
    type Telemetry = CosmosTelemetry;

    fn telemetry(&self) -> &CosmosTelemetry {
        &self.telemetry
    }
}

impl GrpcAddressGetter<CosmosChain> for CosmosChainContextComponents {
    fn grpc_address(chain: &CosmosChain) -> &Uri {
        &chain.tx_config.grpc_address
    }
}

impl RpcClientGetter<CosmosChain> for CosmosChainContextComponents {
    fn rpc_client(chain: &CosmosChain) -> &HttpClient {
        &chain.rpc_client
    }

    fn rpc_address(chain: &CosmosChain) -> &Url {
        &chain.tx_config.rpc_address
    }
}

impl ChainIdGetter<CosmosChain> for CosmosChainContextComponents {
    fn chain_id(chain: &CosmosChain) -> &ChainId {
        &chain.chain_id
    }
}

impl HasEventSubscription for CosmosChain {
    fn event_subscription(&self) -> &Arc<dyn Subscription<Item = (Height, Arc<AbciEvent>)>> {
        &self.subscription
    }
}

pub trait CanUseCosmosChain:
    HasClientStateType<CosmosChain, ClientState = TendermintClientState>
    + HasChannelEndType<CosmosChain, ChannelEnd = ChannelEnd>
    + HasCommitmentProofType<CommitmentProof = CosmosCommitmentProof>
    + HasMessageResponseType<MessageResponse = Vec<Arc<AbciEvent>>>
    + HasEventType<Event = Arc<AbciEvent>>
    + HasCreateClientPayloadType<CosmosChain, CreateClientPayload = CosmosCreateClientPayload>
    + HasUpdateClientPayloadType<CosmosChain, UpdateClientPayload = CosmosUpdateClientPayload>
    + HasCreateClientPayloadOptionsType<
        CosmosChain,
        CreateClientPayloadOptions = CosmosCreateClientOptions,
    > + CanQueryBalance
    + CanIbcTransferToken<CosmosChain>
    + CanConvertGasToFee
    + CanQueryEipBaseFee
    + CanQueryUnbondingPeriod
    + CanBuildIbcTokenTransferMessage<CosmosChain>
    + CanQueryClientState<CosmosChain>
    + CanQueryClientStateWithProofs<CosmosChain>
    + CanQueryConsensusState<CosmosChain>
    + CanQueryConsensusStateWithProofs<CosmosChain>
    + CanQueryRawConsensusState<CosmosChain>
    + CanQueryAllClientStates<CosmosChain>
    + CanQueryClientState<AnyCounterparty>
    + CanQueryConsensusState<AnyCounterparty>
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
    + CanUploadWasmClientCode
    + CanQueryProposalStatus
    + CanBuildDepositProposalMessage
    + CanBuildVoteProposalMessage
    + HasMessageResponseEvents
    + HasSendPacketEvent<CosmosChain>
    + CanBuildCreateClientPayload<CosmosChain>
where
    CosmosChain: HasClientStateType<Self>
        + HasConsensusStateType<Self>
        + HasCreateClientPayloadType<Self>
        + HasUpdateClientPayloadType<Self>,
{
}

impl CanUseCosmosChain for CosmosChain {}

pub trait CanUseLoggerWithCosmosChain:
    for<'a> CanLog<LogSendMessagesWithSignerAndNonce<'a, CosmosChain>>
    + for<'a> CanLog<TxNoResponseError<'a, CosmosChain>>
{
}

impl CanUseLoggerWithCosmosChain for HermesLogger {}
