use core::ops::Deref;
use core::time::Duration;

use cgp::core::error::{ErrorRaiserComponent, ErrorTypeProviderComponent, ErrorWrapperComponent};
use hermes_any_counterparty::contexts::AnyCounterparty;
use hermes_core::chain_components::traits::BlockTimeQuerierComponent;
use hermes_core::encoding_components::traits::{
    DefaultEncodingGetterComponent, EncodingGetterComponent, EncodingTypeProviderComponent,
    HasDefaultEncoding,
};
use hermes_core::encoding_components::types::AsBytes;
use hermes_core::logging_components::traits::LoggerComponent;
use hermes_core::relayer_components::chain::traits::{
    CanBuildAckPacketMessage, CanBuildAckPacketPayload, CanBuildChannelOpenAckMessage,
    CanBuildChannelOpenAckPayload, CanBuildChannelOpenConfirmMessage,
    CanBuildChannelOpenConfirmPayload, CanBuildChannelOpenInitMessage,
    CanBuildChannelOpenTryMessage, CanBuildChannelOpenTryPayload, CanBuildConnectionOpenAckMessage,
    CanBuildConnectionOpenAckPayload, CanBuildConnectionOpenConfirmMessage,
    CanBuildConnectionOpenConfirmPayload, CanBuildConnectionOpenInitMessage,
    CanBuildConnectionOpenInitPayload, CanBuildConnectionOpenTryMessage,
    CanBuildConnectionOpenTryPayload, CanBuildCreateClientMessage, CanBuildReceivePacketMessage,
    CanBuildReceivePacketPayload, CanBuildTimeoutUnorderedPacketMessage,
    CanBuildTimeoutUnorderedPacketPayload, CanBuildUpdateClientMessage, CanQueryAllClientStates,
    CanQueryChannelEnd, CanQueryChannelEndWithProofs, CanQueryClientState,
    CanQueryClientStateWithProofs, CanQueryConnectionEnd, CanQueryConnectionEndWithProofs,
    CanQueryConsensusState, CanQueryConsensusStateWithProofs, CanQueryPacketAckCommitment,
    CanQueryPacketCommitment, CanQueryPacketReceipt, CanQueryRawClientState,
    CanQueryRawConsensusState, ChainIdGetterComponent, HasChannelEndType, HasClientStateType,
    HasConsensusStateType, HasCreateClientMessageOptionsType, HasCreateClientPayloadType,
    HasInitConnectionOptionsType, HasRawClientStateType, HasUpdateClientPayloadType,
    IbcCommitmentPrefixGetter, IbcCommitmentPrefixGetterComponent,
};
use hermes_core::relayer_components::error::traits::{HasRetryableError, RetryableErrorComponent};
use hermes_core::relayer_components::transaction::impls::{
    GetGlobalNonceMutex, GetGlobalSignerMutex, HasPollTimeout, SignerWithIndexGetter,
};
use hermes_core::relayer_components::transaction::traits::{
    CanPollTxResponse, CanQueryTxResponse, CanSubmitTx, ClientRefreshRateGetter,
    ClientRefreshRateGetterComponent, DefaultSignerGetterComponent, FeeForSimulationGetter,
    FeeForSimulationGetterComponent, NonceAllocationMutexGetterComponent, SignerGetterComponent,
    SignerMutexGetterComponent,
};
use hermes_core::relayer_components_extra::telemetry::traits::telemetry::HasTelemetry;
use hermes_core::runtime_components::traits::{
    HasRuntime, RuntimeGetterComponent, RuntimeTypeProviderComponent,
};
use hermes_core::test_components::chain::traits::CanQueryBalance;
use hermes_cosmos_core::chain_components::impls::GetFirstSignerAsDefault;
use hermes_cosmos_core::chain_components::traits::{
    CanQueryAbci, CanQueryUnbondingPeriod, GasConfigGetter, GasConfigGetterComponent,
    GrpcAddressGetter, GrpcAddressGetterComponent, RpcClientGetter, RpcClientGetterComponent,
    TxExtensionOptionsGetter, TxExtensionOptionsGetterComponent,
};
use hermes_cosmos_core::chain_components::types::{
    CosmosCreateClientPayload, CosmosUpdateClientPayload, GasConfig, TendermintClientState,
    TendermintConsensusState, WasmAccessTypeProviderComponent,
};
use hermes_cosmos_core::chain_preset::delegate::DelegateCosmosChainComponents;
use hermes_cosmos_core::tracing_logging_components::contexts::TracingLogger;
use hermes_cosmos_core::wasm_test_components::components::WasmChainComponents;
use hermes_cosmos_core::wasm_test_components::traits::chain::{
    CanUploadWasmClientCode, StoreCodeMessageBuilderComponent, WasmClientCodeUploaderComponent,
};
use hermes_cosmos_relayer::contexts::CosmosChain;
use hermes_cosmos_relayer::impls::HandleCosmosError;
use hermes_cosmos_relayer::types::telemetry::CosmosTelemetry;
use hermes_prelude::*;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_wasm_chain_components::traits::{
    WasmContractInstantiatorComponent, WasmContractUploaderComponent,
};
use ibc::core::channel::types::channel::ChannelEnd;
use ibc_proto::cosmos::tx::v1beta1::Fee;
use prost_types::Any;
use tendermint_rpc::{HttpClient, Url};

use crate::components::{CosmosChainWasmPreset, CosmosToWasmCosmosComponents};
use crate::context::encoding::{UseWasmCosmosEncoding, WasmCosmosEncoding};
use crate::types::WasmTendermintClientState;

#[cgp_context(WasmCosmosChainComponents: CosmosChainWasmPreset)]
#[derive(Clone)]
pub struct WasmCosmosChain {
    pub chain: CosmosChain,
}

impl Deref for WasmCosmosChain {
    type Target = CosmosChain;

    fn deref(&self) -> &CosmosChain {
        &self.chain
    }
}

delegate_components! {
    WasmCosmosChainComponents {
        [
            ErrorTypeProviderComponent,
            ErrorRaiserComponent,
            ErrorWrapperComponent,
            RetryableErrorComponent,
        ]:
            HandleCosmosError,
        RuntimeTypeProviderComponent:
            UseType<HermesRuntime>,
        RuntimeGetterComponent:
            UseField<symbol!("runtime")>,
        LoggerComponent: TracingLogger,
        [
            EncodingTypeProviderComponent<AsBytes>,
            EncodingGetterComponent<AsBytes>,
            DefaultEncodingGetterComponent<AsBytes>,
        ]:
            UseWasmCosmosEncoding,
        [
            StoreCodeMessageBuilderComponent,
            WasmClientCodeUploaderComponent,
            WasmAccessTypeProviderComponent,
            WasmContractUploaderComponent,
            WasmContractInstantiatorComponent,
        ]:
            WasmChainComponents,
        NonceAllocationMutexGetterComponent:
            GetGlobalNonceMutex<symbol!("nonce_mutex")>,
        SignerMutexGetterComponent:
            GetGlobalSignerMutex<symbol!("signer_mutex"), symbol!("key_entries")>,
        DefaultSignerGetterComponent:
            GetFirstSignerAsDefault<symbol!("key_entries")>,
        SignerGetterComponent:
            SignerWithIndexGetter<symbol!("key_entries")>,
        ChainIdGetterComponent:
            UseField<symbol!("chain_id")>,
        BlockTimeQuerierComponent:
            UseField<symbol!("block_time")>,
    }
}

delegate_components! {
    DelegateCosmosChainComponents {
        WasmCosmosChain: CosmosToWasmCosmosComponents::Provider,
    }
}

#[cgp_provider(TxExtensionOptionsGetterComponent)]
impl TxExtensionOptionsGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn tx_extension_options(chain: &WasmCosmosChain) -> &Vec<ibc_proto::google::protobuf::Any> {
        &chain.chain_config.extension_options
    }
}

#[cgp_provider(GasConfigGetterComponent)]
impl GasConfigGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn gas_config(chain: &WasmCosmosChain) -> &GasConfig {
        &chain.chain_config.gas_config
    }
}

#[cgp_provider(FeeForSimulationGetterComponent)]
impl FeeForSimulationGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn fee_for_simulation(chain: &WasmCosmosChain) -> &Fee {
        &chain.chain_config.gas_config.max_fee
    }
}

#[cgp_provider(IbcCommitmentPrefixGetterComponent)]
impl IbcCommitmentPrefixGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn ibc_commitment_prefix(chain: &WasmCosmosChain) -> &Vec<u8> {
        &chain.ibc_commitment_prefix
    }
}

#[cgp_provider(GrpcAddressGetterComponent)]
impl GrpcAddressGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn grpc_address(chain: &WasmCosmosChain) -> &Url {
        &chain.chain_config.grpc_addr
    }
}

#[cgp_provider(RpcClientGetterComponent)]
impl RpcClientGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn rpc_client(chain: &WasmCosmosChain) -> &HttpClient {
        &chain.rpc_client
    }

    fn rpc_address(chain: &WasmCosmosChain) -> &Url {
        &chain.chain_config.rpc_addr
    }
}

#[cgp_provider(ClientRefreshRateGetterComponent)]
impl ClientRefreshRateGetter<WasmCosmosChain> for WasmCosmosChainComponents {
    fn client_refresh_rate(chain: &WasmCosmosChain) -> &Option<Duration> {
        &chain.chain_config.client_refresh_rate
    }
}

impl HasTelemetry for WasmCosmosChain {
    type Telemetry = CosmosTelemetry;

    fn telemetry(&self) -> &CosmosTelemetry {
        &self.telemetry
    }
}

pub trait CanUseWasmCosmosChain:
    HasClientStateType<WasmCosmosChain, ClientState = WasmTendermintClientState>
    + HasConsensusStateType<WasmCosmosChain, ConsensusState = TendermintConsensusState>
    + HasCreateClientPayloadType<WasmCosmosChain, CreateClientPayload = CosmosCreateClientPayload>
    + HasUpdateClientPayloadType<WasmCosmosChain, UpdateClientPayload = CosmosUpdateClientPayload>
    + CanQueryBalance
    // + CanIbcTransferToken<WasmCosmosChain>
    // + CanBuildIbcTokenTransferMessage<WasmCosmosChain>
    + CanQueryRawClientState<WasmCosmosChain>
    + CanQueryClientState<WasmCosmosChain>
    + CanQueryClientStateWithProofs<WasmCosmosChain>
    + CanQueryConsensusState<WasmCosmosChain>
    + CanQueryConsensusStateWithProofs<WasmCosmosChain>
    + CanQueryRawConsensusState<WasmCosmosChain>
    + CanQueryAllClientStates<WasmCosmosChain>
    + CanQueryClientState<AnyCounterparty>
    + CanQueryAllClientStates<AnyCounterparty>
    + CanBuildUpdateClientMessage<WasmCosmosChain>
    + CanQueryConnectionEnd<WasmCosmosChain>
    + CanQueryChannelEnd<WasmCosmosChain>
    + CanQueryChannelEndWithProofs<WasmCosmosChain>
    + CanQueryConnectionEndWithProofs<WasmCosmosChain>
    + CanQueryPacketCommitment<WasmCosmosChain>
    + CanQueryPacketAckCommitment<WasmCosmosChain>
    + CanQueryPacketReceipt<WasmCosmosChain>
    + HasChannelEndType<WasmCosmosChain, ChannelEnd = ChannelEnd>
    + HasRawClientStateType<RawClientState = Any>
    + CanSubmitTx
    + CanPollTxResponse
    + HasPollTimeout
    + CanQueryTxResponse
    + HasRetryableError
    + HasRuntime
    // + CanAssertEventualAmount
    + CanQueryAbci
    + CanQueryUnbondingPeriod
    + CanQueryClientState<CosmosChain>
    + CanQueryConsensusState<CosmosChain>
    + CanBuildConnectionOpenInitPayload<CosmosChain>
    + CanBuildConnectionOpenTryPayload<CosmosChain>
    + CanBuildConnectionOpenAckPayload<CosmosChain>
    + CanBuildConnectionOpenConfirmPayload<CosmosChain>
    + CanBuildConnectionOpenInitMessage<CosmosChain>
    + CanBuildConnectionOpenTryMessage<CosmosChain>
    + CanBuildConnectionOpenAckMessage<CosmosChain>
    + CanBuildConnectionOpenConfirmMessage<CosmosChain>
    + CanBuildChannelOpenTryPayload<CosmosChain>
    + CanBuildChannelOpenAckPayload<CosmosChain>
    + CanBuildChannelOpenConfirmPayload<CosmosChain>
    + CanBuildChannelOpenInitMessage<CosmosChain>
    + CanBuildChannelOpenTryMessage<CosmosChain>
    + CanBuildChannelOpenAckMessage<CosmosChain>
    + CanBuildChannelOpenConfirmMessage<CosmosChain>
    + CanBuildReceivePacketPayload<CosmosChain>
    + CanBuildAckPacketPayload<CosmosChain>
    + CanBuildTimeoutUnorderedPacketPayload<CosmosChain>
    + CanBuildReceivePacketMessage<CosmosChain>
    + CanBuildAckPacketMessage<CosmosChain>
    + CanBuildTimeoutUnorderedPacketMessage<CosmosChain>
    + HasInitConnectionOptionsType<CosmosChain>
    + HasCreateClientMessageOptionsType<
            CosmosChain,
            CreateClientMessageOptions = (),
        >
    + HasDefaultEncoding<AsBytes, Encoding = WasmCosmosEncoding>
    + CanUploadWasmClientCode
where
    CosmosChain: HasClientStateType<Self, ClientState = TendermintClientState>
        + HasConsensusStateType<Self, ConsensusState = TendermintConsensusState>
        + HasUpdateClientPayloadType<Self>
        + HasCreateClientPayloadType<Self>
        ,
    WasmCosmosChain: HasConsensusStateType<Self>
        + HasUpdateClientPayloadType<Self>
        + HasCreateClientPayloadType<Self>
{
}

pub trait CanUseCosmosChainWithWasmCosmosChain:
    CanQueryClientState<WasmCosmosChain>
    + CanQueryConsensusState<WasmCosmosChain>
    + CanBuildConnectionOpenInitPayload<WasmCosmosChain>
    + CanBuildConnectionOpenTryPayload<WasmCosmosChain>
    + CanBuildConnectionOpenAckPayload<WasmCosmosChain>
    + CanBuildConnectionOpenConfirmPayload<WasmCosmosChain>
    + CanBuildConnectionOpenInitMessage<WasmCosmosChain>
    + CanBuildConnectionOpenTryMessage<WasmCosmosChain>
    + CanBuildChannelOpenTryPayload<WasmCosmosChain>
    + CanBuildChannelOpenAckPayload<WasmCosmosChain>
    + CanBuildChannelOpenConfirmPayload<WasmCosmosChain>
    + CanBuildChannelOpenInitMessage<WasmCosmosChain>
    + CanBuildChannelOpenTryMessage<WasmCosmosChain>
    + CanBuildChannelOpenAckMessage<WasmCosmosChain>
    + CanBuildChannelOpenConfirmMessage<WasmCosmosChain>
    + CanBuildReceivePacketPayload<WasmCosmosChain>
    + CanBuildAckPacketPayload<WasmCosmosChain>
    + CanBuildTimeoutUnorderedPacketPayload<WasmCosmosChain>
    + CanBuildReceivePacketMessage<WasmCosmosChain>
    + CanBuildAckPacketMessage<WasmCosmosChain>
    + CanBuildTimeoutUnorderedPacketMessage<WasmCosmosChain>
    + HasInitConnectionOptionsType<WasmCosmosChain>
    + CanBuildCreateClientMessage<WasmCosmosChain>
where
    WasmCosmosChain: HasConsensusStateType<Self>
        + HasCreateClientPayloadType<Self>
        + HasUpdateClientPayloadType<Self>,
{
}

impl CanUseCosmosChainWithWasmCosmosChain for CosmosChain {}
