use cgp_core::prelude::*;
use cgp_core::{delegate_all, ErrorRaiserComponent, ErrorTypeComponent, ProvideInner};
use cgp_error_eyre::{ProvideEyreError, RaiseDebugError};
use eyre::Error;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_encoding_components::traits::has_encoding::{
    DefaultEncodingGetterComponent, EncodingGetterComponent, EncodingTypeComponent, HasEncoding,
};
use hermes_logging_components::traits::has_logger::{
    GlobalLoggerGetterComponent, LoggerGetterComponent, LoggerTypeComponent,
};
use hermes_relayer_components::chain::impls::wait_chain_reach_height::CanWaitChainReachHeight;
use hermes_relayer_components::chain::traits::message_builders::create_client::CanBuildCreateClientMessage;
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::CanBuildConnectionHandshakePayloads;
use hermes_relayer_components::chain::traits::payload_builders::update_client::CanBuildUpdateClientPayload;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_relayer_components::chain::traits::queries::consensus_state::CanQueryConsensusState;
use hermes_relayer_components::chain::traits::queries::consensus_state_height::CanQueryConsensusStateHeight;
use hermes_relayer_components::chain::traits::send_message::{CanSendMessages, MessageSender};
use hermes_relayer_components::chain::traits::types::chain_id::{
    ChainIdGetter, HasChainId, HasChainIdType,
};
use hermes_relayer_components::chain::traits::types::client_state::{
    HasClientStateFields, HasClientStateType,
};
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientEvent, HasCreateClientOptionsType,
};
use hermes_relayer_components::chain::traits::types::height::HasHeightFields;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetter, RuntimeTypeComponent};
use hermes_sovereign_chain_components::sovereign::components::{
    IsSovereignChainClientComponent, SovereignChainClientComponents,
};
use hermes_sovereign_chain_components::sovereign::traits::chain::data_chain::{
    DataChainGetter, DataChainGetterComponent, DataChainTypeComponent, HasDataChain,
    ProvideDataChainType,
};
use hermes_sovereign_chain_components::sovereign::types::client_state::SovereignClientState;
use hermes_sovereign_chain_components::sovereign::types::consensus_state::SovereignConsensusState;
use hermes_sovereign_rollup_components::types::event::SovereignEvent;
use hermes_sovereign_rollup_components::types::height::RollupHeight;
use hermes_sovereign_rollup_components::types::message::SovereignMessage;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;

use crate::contexts::encoding::{ProvideSovereignEncoding, SovereignEncoding};
use crate::contexts::logger::ProvideSovereignLogger;
use crate::contexts::sovereign_rollup::SovereignRollup;

#[derive(Clone)]
pub struct SovereignChain {
    pub runtime: HermesRuntime,
    pub data_chain: CosmosChain,
    pub rollup: SovereignRollup,
}

pub struct SovereignChainComponents;

impl HasComponents for SovereignChain {
    type Components = SovereignChainComponents;
}

delegate_all!(
    IsSovereignChainClientComponent,
    SovereignChainClientComponents,
    SovereignChainComponents,
);

pub struct SovereignDataChainType;

impl<Chain> ProvideDataChainType<Chain> for SovereignDataChainType
where
    Chain: Async,
{
    type DataChain = CosmosChain;
}

impl DataChainGetter<SovereignChain> for SovereignDataChainType {
    fn data_chain(chain: &SovereignChain) -> &CosmosChain {
        &chain.data_chain
    }
}

delegate_components! {
    SovereignChainComponents {
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
        RuntimeTypeComponent: ProvideHermesRuntime,
        [
            DataChainTypeComponent,
            DataChainGetterComponent,
        ]: SovereignDataChainType,
        [
            EncodingTypeComponent,
            EncodingGetterComponent,
            DefaultEncodingGetterComponent,
        ]:
            ProvideSovereignEncoding,
        [
            LoggerTypeComponent,
            LoggerGetterComponent,
            GlobalLoggerGetterComponent,
        ]:
            ProvideSovereignLogger,
    }
}

impl ProvideInner<SovereignChain> for SovereignChainComponents {
    type Inner = SovereignRollup;

    fn inner(chain: &SovereignChain) -> &SovereignRollup {
        &chain.rollup
    }
}

impl RuntimeGetter<SovereignChain> for SovereignChainComponents {
    fn runtime(chain: &SovereignChain) -> &HermesRuntime {
        &chain.runtime
    }
}

impl ChainIdGetter<SovereignChain> for SovereignChainComponents {
    fn chain_id(chain: &SovereignChain) -> &ChainId {
        chain.data_chain.chain_id()
    }
}

impl MessageSender<SovereignChain> for SovereignChainComponents {
    async fn send_messages(
        chain: &SovereignChain,
        messages: Vec<SovereignMessage>,
    ) -> Result<Vec<Vec<SovereignEvent>>, Error> {
        chain.rollup.send_messages(messages).await
    }
}

pub trait CanUseSovereignChain:
    HasDataChain
    + HasChainIdType
    + HasUpdateClientPayloadType<CosmosChain>
    + HasHeightFields<Height = RollupHeight>
    + CanSendMessages
    + CanQueryChainStatus
    + CanWaitChainReachHeight
    + HasClientStateType<CosmosChain, ClientState = SovereignClientState>
    + HasConsensusStateType<CosmosChain, ConsensusState = SovereignConsensusState>
    + CanBuildUpdateClientPayload<CosmosChain>
    + HasEncoding<Encoding = SovereignEncoding>
    + CanBuildConnectionHandshakePayloads<CosmosChain>
    + CanBuildCreateClientMessage<CosmosChain>
    + HasCreateClientOptionsType<CosmosChain>
    + HasCreateClientEvent<CosmosChain>
    + CanQueryClientState<CosmosChain>
    + CanQueryConsensusState<CosmosChain>
    + CanQueryConsensusStateHeight<CosmosChain>
    + HasClientStateFields<CosmosChain>
{
}

impl CanUseSovereignChain for SovereignChain {}
