use alloc::sync::Arc;
use core::ops::Deref;
use core::time::Duration;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::core::field::WithField;
use cgp::core::types::WithType;
use cgp::prelude::*;
use futures::lock::Mutex;
use hermes_any_counterparty::contexts::any_counterparty::AnyCounterparty;
use hermes_chain_type_components::traits::fields::chain_id::ChainIdGetterComponent;
use hermes_chain_type_components::traits::fields::message_response_events::MessageResponseEventsGetterComponent;
use hermes_chain_type_components::traits::types::event::HasEventType;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_cosmos_chain_components::components::cosmos_to_cosmos::CosmosToCosmosComponents;
use hermes_cosmos_chain_components::components::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_chain_components::impls::types::config::CosmosChainConfig;
use hermes_cosmos_chain_components::traits::eip::eip_query::EipQuerierComponent;
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
use hermes_cosmos_chain_components::traits::unbonding_period::UnbondingPeriodQuerierComponent;
use hermes_cosmos_chain_components::types::commitment_proof::CosmosCommitmentProof;
use hermes_cosmos_chain_components::types::config::gas::gas_config::GasConfig;
use hermes_cosmos_chain_components::types::events::channel::{
    CosmosChannelOpenInitEvent, CosmosChannelOpenTryEvent,
};
use hermes_cosmos_chain_components::types::events::client::CosmosCreateClientEvent;
use hermes_cosmos_chain_components::types::events::connection::{
    CosmosConnectionOpenInitEvent, CosmosConnectionOpenTryEvent,
};
use hermes_cosmos_chain_components::types::key_types::secp256k1::Secp256k1KeyPair;
use hermes_cosmos_chain_components::types::messages::packet::packet_filter::PacketFilterConfig;
use hermes_cosmos_chain_components::types::payloads::client::{
    CosmosCreateClientOptions, CosmosCreateClientPayload, CosmosUpdateClientPayload,
};
use hermes_cosmos_chain_components::types::tendermint::TendermintClientState;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetterComponent, EncodingGetterComponent, EncodingTypeComponent,
};
use hermes_logger::{HermesLogger, UseHermesLogger};
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeProviderComponent,
};
use hermes_logging_components::traits::logger::LoggerComponent;
use hermes_relayer_components::chain::traits::commitment_prefix::{
    HasCommitmentPrefixType, IbcCommitmentPrefixGetter, IbcCommitmentPrefixGetterComponent,
};
use hermes_relayer_components::chain::traits::extract_data::{
    CanExtractFromMessageResponse, EventExtractorComponent, MessageResponseExtractorComponent,
};
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilderComponent;
use hermes_relayer_components::chain::traits::packet::filter::{
    IncomingPacketFilterComponent, OutgoingPacketFilterComponent,
};
use hermes_relayer_components::chain::traits::payload_builders::create_client::CreateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::payload_builders::update_client::UpdateClientPayloadBuilderComponent;
use hermes_relayer_components::chain::traits::queries::block_events::BlockEventsQuerierComponent;
use hermes_relayer_components::chain::traits::queries::block_time::BlockTimeQuerierComponent;
use hermes_relayer_components::chain::traits::queries::chain_status::ChainStatusQuerierComponent;
use hermes_relayer_components::chain::traits::queries::channel_end::{
    ChannelEndQuerierComponent, ChannelEndWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::client_state::{
    AllClientStatesQuerierComponent, ClientStateQuerierComponent,
    ClientStateWithProofsQuerierComponent, RawClientStateQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::connection_end::{
    ConnectionEndQuerierComponent, ConnectionEndWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    ConsensusStateQuerierComponent, ConsensusStateWithProofsQuerierComponent,
};
use hermes_relayer_components::chain::traits::queries::packet_acknowledgement::PacketAcknowledgementQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_commitment::PacketCommitmentQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_is_cleared::PacketIsClearedQuerierComponent;
use hermes_relayer_components::chain::traits::queries::packet_receipt::PacketReceiptQuerierComponent;
use hermes_relayer_components::chain::traits::types::chain_id::ChainIdGetter;
use hermes_relayer_components::chain::traits::types::channel::HasChannelEndType;
use hermes_relayer_components::chain::traits::types::client_state::{
    HasClientStateType, HasRawClientStateType,
};
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientMessageOptionsType, HasCreateClientPayloadOptionsType,
    HasCreateClientPayloadType,
};
use hermes_relayer_components::chain::traits::types::ibc_events::send_packet::SendPacketEventComponent;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use hermes_relayer_components::error::traits::RetryableErrorComponent;
use hermes_relayer_components::transaction::impls::estimate_fees_and_send_tx::LogSendMessagesWithSignerAndNonce;
use hermes_relayer_components::transaction::impls::global_nonce_mutex::GetGlobalNonceMutex;
use hermes_relayer_components::transaction::impls::poll_tx_response::TxNoResponseError;
use hermes_relayer_components::transaction::traits::default_signer::{
    DefaultSignerGetter, DefaultSignerGetterComponent,
};
use hermes_relayer_components::transaction::traits::nonce::nonce_mutex::NonceAllocationMutexGetterComponent;
use hermes_relayer_components::transaction::traits::poll_tx_response::TxResponsePollerComponent;
use hermes_relayer_components::transaction::traits::query_tx_response::TxResponseQuerierComponent;
use hermes_relayer_components::transaction::traits::simulation_fee::{
    FeeForSimulationGetter, FeeForSimulationGetterComponent,
};
use hermes_relayer_components::transaction::traits::submit_tx::TxSubmitterComponent;
use hermes_relayer_components_extra::telemetry::traits::telemetry::HasTelemetry;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_test_components::chain::traits::assert::eventual_amount::EventualAmountAsserterComponent;
use hermes_test_components::chain::traits::messages::ibc_transfer::{
    CanBuildIbcTokenTransferMessage, IbcTokenTransferMessageBuilderComponent,
};
use hermes_test_components::chain::traits::proposal::query_status::ProposalStatusQuerierComponent;
use hermes_test_components::chain::traits::queries::balance::BalanceQuerierComponent;
use hermes_test_components::chain::traits::transfer::ibc_transfer::TokenIbcTransferrerComponent;
use hermes_wasm_test_components::components::WasmChainComponents;
use hermes_wasm_test_components::traits::chain::messages::store_code::StoreCodeMessageBuilderComponent;
use hermes_wasm_test_components::traits::chain::upload_client_code::WasmClientCodeUploaderComponent;
use ibc::core::channel::types::channel::ChannelEnd;
use ibc::core::host::types::identifiers::ChainId;
use ibc_proto::cosmos::tx::v1beta1::Fee;
use prost_types::Any;
use tendermint::abci::Event as AbciEvent;
use tendermint_rpc::client::CompatMode;
use tendermint_rpc::{HttpClient, Url};

use crate::contexts::encoding::ProvideCosmosEncoding;
use crate::impls::error::HandleCosmosError;
use crate::presets::chain::CosmosChainFullPreset;
use crate::types::telemetry::CosmosTelemetry;

#[cgp_context(CosmosChainContextComponents: CosmosChainFullPreset)]
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
    pub block_time: Duration,
    pub nonce_mutex: Arc<Mutex<()>>,
}

impl Deref for CosmosChain {
    type Target = BaseCosmosChain;

    fn deref(&self) -> &BaseCosmosChain {
        &self.base_chain
    }
}

delegate_components! {
    CosmosChainContextComponents {
        [
            ErrorTypeProviderComponent,
            ErrorRaiserComponent,
            ErrorWrapperComponent,
            RetryableErrorComponent,
        ]:
            HandleCosmosError,
        RuntimeTypeProviderComponent: WithType<HermesRuntime>,
        RuntimeGetterComponent: WithField<symbol!("runtime")>,
        [
            LoggerTypeProviderComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            UseHermesLogger,
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

        NonceAllocationMutexGetterComponent:
            GetGlobalNonceMutex<symbol!("nonce_mutex")>,
        BlockTimeQuerierComponent:
            UseField<symbol!("block_time")>,
    }
}

delegate_components! {
    DelegateCosmosChainComponents {
        CosmosChain: CosmosToCosmosComponents::Provider,
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

        let block_time = chain_config.block_time;

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
                nonce_mutex: Arc::new(Mutex::new(())),
                packet_filter,
                block_time,
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
    > + HasCommitmentPrefixType<CommitmentPrefix = Vec<u8>>
    + CanBuildIbcTokenTransferMessage<CosmosChain>
    + HasRawClientStateType<RawClientState = Any>
    + CanExtractFromMessageResponse<CosmosCreateClientEvent>
{
}

impl CanUseCosmosChain for CosmosChain {}

check_components! {
    CanUseCosmosChainComponents for CosmosChain {
        TxSubmitterComponent,
        TxResponsePollerComponent,
        TxResponseQuerierComponent,
        ChainStatusQuerierComponent,
        BlockEventsQuerierComponent,
        BlockTimeQuerierComponent,
        BalanceQuerierComponent,
        ProposalStatusQuerierComponent,
        EipQuerierComponent,
        UnbondingPeriodQuerierComponent,

        IbcCommitmentPrefixGetterComponent,
        MessageResponseEventsGetterComponent,

        WasmClientCodeUploaderComponent,
        EventualAmountAsserterComponent,

        [
            EventExtractorComponent,
            MessageResponseExtractorComponent,
        ]: [
            CosmosCreateClientEvent,
            CosmosConnectionOpenInitEvent,
            CosmosConnectionOpenTryEvent,
            CosmosChannelOpenInitEvent,
            CosmosChannelOpenTryEvent,
        ],

        [
            ClientStateWithProofsQuerierComponent,
            ConsensusStateWithProofsQuerierComponent,
            ConnectionEndQuerierComponent,
            ConnectionEndWithProofsQuerierComponent,
            ChannelEndQuerierComponent,
            ChannelEndWithProofsQuerierComponent,
            PacketCommitmentQuerierComponent,
            PacketAcknowledgementQuerierComponent,
            PacketReceiptQuerierComponent,

            CreateClientPayloadBuilderComponent,
            CreateClientMessageBuilderComponent,
            UpdateClientMessageBuilderComponent,
            UpdateClientPayloadBuilderComponent,
            IbcTokenTransferMessageBuilderComponent,

            IncomingPacketFilterComponent,
            OutgoingPacketFilterComponent,

            TokenIbcTransferrerComponent,
        ]:
            CosmosChain,
        [
            ClientStateQuerierComponent,
            AllClientStatesQuerierComponent,
            ConsensusStateQuerierComponent,
        ]: [
            CosmosChain,
            AnyCounterparty,
        ]
    }
}

check_components! {
    <Counterparty>
    CanUseCosmosChainWithAnyCounterparty for CosmosChain
    {
        [
            PacketIsClearedQuerierComponent,
            RawClientStateQuerierComponent,
            ChannelEndQuerierComponent,

            SendPacketEventComponent,
        ]: Counterparty,
    }
}

check_components! {
    <'a>
    CanLogWithCosmosChain for HermesLogger
    {
        LoggerComponent: [
            LogSendMessagesWithSignerAndNonce<'a, CosmosChain>,
            TxNoResponseError<'a, CosmosChain>,
        ]
    }
}
