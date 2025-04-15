use alloc::sync::Arc;
use core::ops::Deref;
use core::time::Duration;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use cgp::prelude::*;
use futures::lock::Mutex;
use hermes_any_counterparty::contexts::any_counterparty::AnyCounterparty;
use hermes_chain_type_components::traits::{
    ChainIdGetterComponent, HasEventType, HasMessageResponseType,
    MessageResponseEventsGetterComponent,
};
use hermes_cosmos_chain_components::impls::CosmosChainConfig;
use hermes_cosmos_chain_components::traits::{
    EipQuerierComponent, GasConfigGetter, GasConfigGetterComponent, GrpcAddressGetter,
    GrpcAddressGetterComponent, RpcClientGetter, RpcClientGetterComponent,
    TxExtensionOptionsGetter, TxExtensionOptionsGetterComponent, UnbondingPeriodQuerierComponent,
};
use hermes_cosmos_chain_components::types::{
    CosmosChannelOpenInitEvent, CosmosChannelOpenTryEvent, CosmosCommitmentProof,
    CosmosConnectionOpenInitEvent, CosmosConnectionOpenTryEvent, CosmosCreateClientEvent,
    CosmosCreateClientOptions, CosmosCreateClientPayload, CosmosUpdateClientPayload, GasConfig,
    PacketFilterConfig, Secp256k1KeyPair, TendermintClientState,
};
use hermes_cosmos_chain_preset::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_chain_preset::presets::{CosmosChainPreset, CosmosToCosmosComponents};
use hermes_encoding_components::traits::{
    DefaultEncodingGetterComponent, EncodingGetterComponent, EncodingTypeProviderComponent,
};
use hermes_encoding_components::types::AsBytes;
use hermes_logging_components::traits::logger::LoggerComponent;
use hermes_relayer_components::chain::traits::{
    AllClientStatesQuerierComponent, BlockEventsQuerierComponent, BlockTimeQuerierComponent,
    CanExtractFromMessageResponse, ChainStatusQuerierComponent, ChannelEndQuerierComponent,
    ChannelEndWithProofsQuerierComponent, ClientStateQuerierComponent,
    ClientStateWithProofsQuerierComponent, ConnectionEndQuerierComponent,
    ConnectionEndWithProofsQuerierComponent, ConsensusStateQuerierComponent,
    ConsensusStateWithProofsQuerierComponent, CreateClientMessageBuilderComponent,
    CreateClientPayloadBuilderComponent, EventExtractorComponent, HasChannelEndType,
    HasClientStateType, HasCommitmentPrefixType, HasCommitmentProofType,
    HasCreateClientMessageOptionsType, HasCreateClientPayloadOptionsType,
    HasCreateClientPayloadType, HasRawClientStateType, HasUpdateClientPayloadType,
    IbcCommitmentPrefixGetter, IbcCommitmentPrefixGetterComponent, IncomingPacketFilterComponent,
    MessageResponseExtractorComponent, OutgoingPacketFilterComponent,
    PacketAckCommitmentQuerierComponent, PacketCommitmentQuerierComponent,
    PacketIsClearedQuerierComponent, PacketReceiptQuerierComponent, RawClientStateQuerierComponent,
    SendPacketEventComponent, UpdateClientMessageBuilderComponent,
    UpdateClientPayloadBuilderComponent,
};
use hermes_relayer_components::error::traits::RetryableErrorComponent;
use hermes_relayer_components::transaction::impls::{
    GetGlobalNonceMutex, LogSendMessagesWithSignerAndNonce, TxNoResponseError,
};
use hermes_relayer_components::transaction::traits::{
    DefaultSignerGetterComponent, FeeForSimulationGetter, FeeForSimulationGetterComponent,
    NonceAllocationMutexGetterComponent, TxResponsePollerComponent, TxResponseQuerierComponent,
    TxSubmitterComponent,
};
use hermes_relayer_components_extra::telemetry::traits::telemetry::HasTelemetry;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{
    RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_test_components::chain::traits::{
    BalanceQuerierComponent, CanBuildIbcTokenTransferMessages, EventualAmountAsserterComponent,
    IbcTokenTransferMessageBuilderComponent, ProposalStatusQuerierComponent,
    TokenIbcTransferrerComponent,
};
use hermes_tracing_logging_components::contexts::logger::TracingLogger;
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

use crate::contexts::UseCosmosEncoding;
use crate::impls::HandleCosmosError;
use crate::types::telemetry::CosmosTelemetry;

#[cgp_context(CosmosChainContextComponents: CosmosChainPreset)]
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
        RuntimeTypeProviderComponent: UseType<HermesRuntime>,
        RuntimeGetterComponent: UseField<symbol!("runtime")>,
        LoggerComponent: TracingLogger,
        [
            EncodingTypeProviderComponent<AsBytes>,
            EncodingGetterComponent<AsBytes>,
            DefaultEncodingGetterComponent<AsBytes>,
        ]:
            UseCosmosEncoding,
        [
            StoreCodeMessageBuilderComponent,
            WasmClientCodeUploaderComponent,
        ]:
            WasmChainComponents,

        NonceAllocationMutexGetterComponent:
            GetGlobalNonceMutex<symbol!("nonce_mutex")>,
        BlockTimeQuerierComponent:
            UseField<symbol!("block_time")>,
        DefaultSignerGetterComponent:
            UseField<symbol!("key_entry")>,
        ChainIdGetterComponent:
            UseField<symbol!("chain_id")>,
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
    + CanBuildIbcTokenTransferMessages<CosmosChain>
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
            PacketAckCommitmentQuerierComponent,
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
    CanLogWithCosmosChain for CosmosChain
    {
        LoggerComponent: [
            LogSendMessagesWithSignerAndNonce<'a, CosmosChain>,
            TxNoResponseError<'a, CosmosChain>,
        ]
    }
}
