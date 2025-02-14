use alloc::sync::Arc;
use core::ops::Deref;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeComponent};
use cgp::core::field::WithField;
use cgp::core::types::WithType;
use cgp::prelude::*;
use futures::lock::Mutex;
use hermes_any_counterparty::contexts::any_counterparty::AnyCounterparty;
use hermes_chain_type_components::traits::fields::chain_id::ChainIdGetterComponent;
use hermes_chain_type_components::traits::fields::message_response_events::HasMessageResponseEvents;
use hermes_chain_type_components::traits::types::event::HasEventType;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_cosmos_chain_components::components::cosmos_to_cosmos::CosmosToCosmosComponents;
use hermes_cosmos_chain_components::components::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_chain_components::impls::types::config::CosmosChainConfig;
use hermes_cosmos_chain_components::traits::convert_gas_to_fee::CanConvertGasToFee;
use hermes_cosmos_chain_components::traits::eip::eip_query::CanQueryEipBaseFee;
use hermes_cosmos_chain_components::traits::gas_config::{
    GasConfigGetter, GasConfigGetterComponent,
};
use hermes_cosmos_chain_components::traits::grpc_address::{
    GrpcAddressGetter, GrpcAddressGetterComponent,
};
use hermes_cosmos_chain_components::traits::rpc_client::{
    RpcClientGetter, RpcClientGetterComponent,
};
use hermes_cosmos_chain_components::traits::tx_extension_options::{
    TxExtensionOptionsGetter, TxExtensionOptionsGetterComponent,
};
use hermes_cosmos_chain_components::traits::unbonding_period::CanQueryUnbondingPeriod;
use hermes_cosmos_chain_components::types::commitment_proof::CosmosCommitmentProof;
use hermes_cosmos_chain_components::types::config::gas::gas_config::GasConfig;
use hermes_cosmos_chain_components::types::events::client::CosmosCreateClientEvent;
use hermes_cosmos_chain_components::types::key_types::secp256k1::Secp256k1KeyPair;
use hermes_cosmos_chain_components::types::messages::packet::packet_filter::PacketFilterConfig;
use hermes_cosmos_chain_components::types::nonce_guard::NonceGuard;
use hermes_cosmos_chain_components::types::payloads::client::{
    CosmosCreateClientOptions, CosmosCreateClientPayload, CosmosUpdateClientPayload,
};
use hermes_cosmos_chain_components::types::tendermint::TendermintClientState;
use hermes_cosmos_chain_components::types::transaction::account::Account;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetterComponent, EncodingGetterComponent, EncodingTypeComponent,
};
use hermes_logger::{HermesLogger, ProvideHermesLogger};
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_logging_components::traits::logger::CanLog;
use hermes_relayer_components::chain::traits::commitment_prefix::{
    HasCommitmentPrefixType, HasIbcCommitmentPrefix, IbcCommitmentPrefixGetter,
    IbcCommitmentPrefixGetterComponent,
};
use hermes_relayer_components::chain::traits::extract_data::{
    CanExtractFromEvent, CanExtractFromMessageResponse,
};
use hermes_relayer_components::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use hermes_relayer_components::chain::traits::packet::fields::HasPacketSrcChannelId;
use hermes_relayer_components::chain::traits::packet::filter::{
    CanFilterIncomingPacket, CanFilterOutgoingPacket,
};
use hermes_relayer_components::chain::traits::payload_builders::create_client::CanBuildCreateClientPayload;
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
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientMessageOptionsType, HasCreateClientPayloadOptionsType,
    HasCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::HasSendPacketEvent;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use hermes_relayer_components::error::traits::retry::RetryableErrorComponent;
use hermes_relayer_components::transaction::impls::estimate_fees_and_send_tx::LogSendMessagesWithSignerAndNonce;
use hermes_relayer_components::transaction::impls::poll_tx_response::TxNoResponseError;
use hermes_relayer_components::transaction::traits::default_signer::{
    DefaultSignerGetter, DefaultSignerGetterComponent,
};
use hermes_relayer_components::transaction::traits::nonce::nonce_mutex::{
    MutexForNonceAllocationComponent, ProvideMutexForNonceAllocation,
};
use hermes_relayer_components::transaction::traits::poll_tx_response::CanPollTxResponse;
use hermes_relayer_components::transaction::traits::query_tx_response::CanQueryTxResponse;
use hermes_relayer_components::transaction::traits::simulation_fee::{
    FeeForSimulationGetter, FeeForSimulationGetterComponent,
};
use hermes_relayer_components::transaction::traits::submit_tx::CanSubmitTx;
use hermes_relayer_components_extra::telemetry::traits::telemetry::HasTelemetry;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::mutex::MutexGuardOf;
use hermes_runtime_components::traits::runtime::{RuntimeGetterComponent, RuntimeTypeComponent};
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
use ibc::core::channel::types::channel::ChannelEnd;
use ibc::core::host::types::identifiers::ChainId;
use ibc_proto::cosmos::tx::v1beta1::Fee;
use prost_types::Any;
use tendermint::abci::Event as AbciEvent;
use tendermint_rpc::client::CompatMode;
use tendermint_rpc::{HttpClient, Url};

use crate::contexts::encoding::ProvideCosmosEncoding;
use crate::impls::error::HandleCosmosError;
use crate::presets::chain::*;
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
    pub ibc_commitment_prefix: Vec<u8>,
    pub rpc_client: HttpClient,
    pub key_entry: Secp256k1KeyPair,
    pub packet_filter: PacketFilterConfig,
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
        RuntimeTypeComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
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

impl<Name> DelegateComponent<Name> for CosmosChainContextComponents
where
    Self: IsCosmosChainFullPreset<Name>,
{
    type Delegate = CosmosChainFullPreset;
}

impl<Name, Context, Params> IsProviderFor<Name, Context, Params> for CosmosChainContextComponents
where
    Self: IsCosmosChainFullPreset<Name>,
    CosmosChainFullPreset: IsProviderFor<Name, Context, Params>,
{
}

delegate_components! {
    DelegateCosmosChainComponents {
        CosmosChain: CosmosToCosmosComponents,
    }
}

#[cgp_provider(TxExtensionOptionsGetterComponent)]
impl TxExtensionOptionsGetter<CosmosChain> for CosmosChainContextComponents {
    fn tx_extension_options(chain: &CosmosChain) -> &Vec<ibc_proto::google::protobuf::Any> {
        &chain.chain_config.extension_options
    }
}

#[cgp_provider(GasConfigGetterComponent)]
impl GasConfigGetter<CosmosChain> for CosmosChainContextComponents {
    fn gas_config(chain: &CosmosChain) -> &GasConfig {
        &chain.chain_config.gas_config
    }
}

#[cgp_provider(DefaultSignerGetterComponent)]
impl DefaultSignerGetter<CosmosChain> for CosmosChainContextComponents {
    fn get_default_signer(chain: &CosmosChain) -> &Secp256k1KeyPair {
        &chain.key_entry
    }
}

#[cgp_provider(FeeForSimulationGetterComponent)]
impl FeeForSimulationGetter<CosmosChain> for CosmosChainContextComponents {
    fn fee_for_simulation(chain: &CosmosChain) -> &Fee {
        &chain.chain_config.gas_config.max_fee
    }
}

#[cgp_provider(MutexForNonceAllocationComponent)]
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

#[cgp_provider(IbcCommitmentPrefixGetterComponent)]
impl IbcCommitmentPrefixGetter<CosmosChain> for CosmosChainContextComponents {
    fn ibc_commitment_prefix(chain: &CosmosChain) -> &Vec<u8> {
        &chain.ibc_commitment_prefix
    }
}

impl CosmosChain {
    pub fn new(
        chain_config: CosmosChainConfig,
        rpc_client: HttpClient,
        compat_mode: CompatMode,
        key_entry: Secp256k1KeyPair,
        runtime: HermesRuntime,
        telemetry: CosmosTelemetry,
        packet_filter: PacketFilterConfig,
    ) -> Self {
        let chain_id = ChainId::new(&chain_config.id).unwrap();

        let ibc_commitment_prefix = chain_config.store_prefix.clone().into();

        let chain = Self {
            base_chain: Arc::new(BaseCosmosChain {
                chain_config,
                chain_id,
                compat_mode,
                runtime,
                telemetry,
                ibc_commitment_prefix,
                rpc_client,
                key_entry,
                nonce_mutex: Mutex::new(()),
                packet_filter,
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

#[cgp_provider(GrpcAddressGetterComponent)]
impl GrpcAddressGetter<CosmosChain> for CosmosChainContextComponents {
    fn grpc_address(chain: &CosmosChain) -> &Url {
        &chain.chain_config.grpc_addr
    }
}

#[cgp_provider(RpcClientGetterComponent)]
impl RpcClientGetter<CosmosChain> for CosmosChainContextComponents {
    fn rpc_client(chain: &CosmosChain) -> &HttpClient {
        &chain.rpc_client
    }

    fn rpc_address(chain: &CosmosChain) -> &Url {
        &chain.chain_config.rpc_addr
    }
}

#[cgp_provider(ChainIdGetterComponent)]
impl ChainIdGetter<CosmosChain> for CosmosChainContextComponents {
    fn chain_id(chain: &CosmosChain) -> &ChainId {
        &chain.chain_id
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
    + HasCreateClientMessageOptionsType<CosmosChain, CreateClientMessageOptions = ()>
    + HasCreateClientPayloadOptionsType<
        CosmosChain,
        CreateClientPayloadOptions = CosmosCreateClientOptions,
    > + CanQueryBalance
    + HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>
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
    + HasIbcCommitmentPrefix
    + HasSendPacketEvent<CosmosChain>
    + CanBuildCreateClientPayload<CosmosChain>
    + CanFilterIncomingPacket<CosmosChain>
    + CanFilterOutgoingPacket<CosmosChain>
    + HasPacketSrcChannelId<CosmosChain>
    + CanExtractFromEvent<CosmosCreateClientEvent>
    + CanExtractFromMessageResponse<CosmosCreateClientEvent>
    + CanUseComponent<ChainStatusQuerierComponent>
    + CanUseComponent<BlockEventsQuerierComponent>
{
}

impl CanUseCosmosChain for CosmosChain {}

pub trait CanUseCosmosChainComponents:
    Async + CanUseComponent<ChainStatusQuerierComponent> // + CanQueryChainStatus
{
}

impl CanUseCosmosChainComponents for CosmosChain {}

pub trait CanUseLoggerWithCosmosChain:
    for<'a> CanLog<LogSendMessagesWithSignerAndNonce<'a, CosmosChain>>
    + for<'a> CanLog<TxNoResponseError<'a, CosmosChain>>
{
}

impl CanUseLoggerWithCosmosChain for HermesLogger {}
