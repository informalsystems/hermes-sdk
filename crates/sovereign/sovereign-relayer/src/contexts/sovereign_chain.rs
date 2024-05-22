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
use hermes_relayer_components::chain::traits::message_builders::channel_handshake::{
    CanBuildChannelOpenAckMessage, CanBuildChannelOpenConfirmMessage, CanBuildChannelOpenTryMessage,
};
use hermes_relayer_components::chain::traits::message_builders::connection_handshake::{
    CanBuildConnectionOpenAckMessage, CanBuildConnectionOpenConfirmMessage,
    CanBuildConnectionOpenInitMessage, CanBuildConnectionOpenTryMessage,
};
use hermes_relayer_components::chain::traits::message_builders::create_client::CanBuildCreateClientMessage;
use hermes_relayer_components::chain::traits::payload_builders::connection_handshake::{
    CanBuildConnectionOpenAckPayload, CanBuildConnectionOpenConfirmPayload,
    CanBuildConnectionOpenInitPayload, CanBuildConnectionOpenTryPayload,
};
use hermes_relayer_components::chain::traits::payload_builders::update_client::CanBuildUpdateClientPayload;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::queries::client_state::{
    CanQueryClientState, CanQueryClientStateWithProofs,
};
use hermes_relayer_components::chain::traits::queries::connection_end::{
    CanQueryConnectionEnd, CanQueryConnectionEndWithProofs,
};
use hermes_relayer_components::chain::traits::queries::consensus_state::{
    CanQueryConsensusState, CanQueryConsensusStateWithProofs,
};
use hermes_relayer_components::chain::traits::queries::consensus_state_height::CanQueryConsensusStateHeight;
use hermes_relayer_components::chain::traits::send_message::{CanSendMessages, MessageSender};
use hermes_relayer_components::chain::traits::types::chain_id::{
    ChainIdGetter, HasChainId, HasChainIdType,
};
use hermes_relayer_components::chain::traits::types::channel::{
    HasChannelEndType, HasInitChannelOptionsType,
};
use hermes_relayer_components::chain::traits::types::client_state::{
    HasClientStateFields, HasClientStateType,
};
use hermes_relayer_components::chain::traits::types::connection::{
    HasConnectionEndType, HasInitConnectionOptionsType,
};
use hermes_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use hermes_relayer_components::chain::traits::types::create_client::{
    HasCreateClientEvent, HasCreateClientOptionsType,
};
use hermes_relayer_components::chain::traits::types::height::{
    CanIncrementHeight, HasHeightFields,
};
use hermes_relayer_components::chain::traits::types::ibc::HasCounterpartyMessageHeight;
use hermes_relayer_components::chain::traits::types::ibc_events::connection::{
    HasConnectionOpenInitEvent, HasConnectionOpenTryEvent,
};
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use hermes_runtime::impls::types::runtime::ProvideHermesRuntime;
use hermes_runtime::types::runtime::HermesRuntime;
use hermes_runtime_components::traits::runtime::{RuntimeGetter, RuntimeTypeComponent};
use hermes_sovereign_chain_components::sovereign::components::{
    IsSovereignChainClientComponent, SovereignChainClientComponents,
};
use hermes_sovereign_chain_components::sovereign::traits::chain::data_chain::{
    DataChainGetter, HasDataChain, ProvideDataChainType,
};
use hermes_sovereign_chain_components::sovereign::traits::chain::rollup::{
    ProvideRollupType, RollupGetter,
};
use hermes_sovereign_rollup_components::types::client_state::WrappedSovereignClientState;
use hermes_sovereign_rollup_components::types::consensus_state::SovereignConsensusState;
use hermes_sovereign_rollup_components::types::event::SovereignEvent;
use hermes_sovereign_rollup_components::types::height::RollupHeight;
use hermes_sovereign_rollup_components::types::message::SovereignMessage;
use ibc::core::channel::types::channel::ChannelEnd;
use ibc::core::connection::types::ConnectionEnd;
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

delegate_components! {
    SovereignChainComponents {
        ErrorTypeComponent: ProvideEyreError,
        ErrorRaiserComponent: RaiseDebugError,
        RuntimeTypeComponent: ProvideHermesRuntime,
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

impl ProvideDataChainType<SovereignChain> for SovereignChainComponents {
    type DataChain = CosmosChain;
}

impl DataChainGetter<SovereignChain> for SovereignChainComponents {
    fn data_chain(chain: &SovereignChain) -> &CosmosChain {
        &chain.data_chain
    }
}

impl ProvideRollupType<SovereignChain> for SovereignChainComponents {
    type Rollup = SovereignRollup;
}

impl RollupGetter<SovereignChain> for SovereignChainComponents {
    fn rollup(chain: &SovereignChain) -> &SovereignRollup {
        &chain.rollup
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
    + HasMessageType<Message = SovereignMessage>
    + HasCommitmentProofType<CommitmentProof = Vec<u8>>
    + CanIncrementHeight
    + CanSendMessages
    + CanQueryChainStatus
    + CanWaitChainReachHeight
    + HasCounterpartyMessageHeight<CosmosChain>
    + HasClientStateType<CosmosChain, ClientState = WrappedSovereignClientState>
    + HasConsensusStateType<CosmosChain, ConsensusState = SovereignConsensusState>
    + HasConnectionEndType<CosmosChain, ConnectionEnd = ConnectionEnd>
    + HasChannelEndType<CosmosChain, ChannelEnd = ChannelEnd>
    + CanBuildUpdateClientPayload<CosmosChain>
    + HasEncoding<Encoding = SovereignEncoding>
    + CanBuildCreateClientMessage<CosmosChain>
    + HasCreateClientOptionsType<CosmosChain>
    + HasCreateClientEvent<CosmosChain>
    + HasConnectionOpenInitEvent<CosmosChain>
    + CanQueryClientState<CosmosChain>
    + CanQueryClientStateWithProofs<CosmosChain>
    + CanQueryConsensusState<CosmosChain>
    + CanQueryConsensusStateWithProofs<CosmosChain>
    + CanQueryConsensusStateHeight<CosmosChain>
    + CanQueryConnectionEnd<CosmosChain>
    + CanQueryConnectionEndWithProofs<CosmosChain>
    + HasClientStateFields<CosmosChain>
    + HasInitConnectionOptionsType<CosmosChain>
    + CanBuildConnectionOpenInitPayload<CosmosChain>
    + CanBuildConnectionOpenTryPayload<CosmosChain>
    + CanBuildConnectionOpenAckPayload<CosmosChain>
    + CanBuildConnectionOpenConfirmPayload<CosmosChain>
    + CanBuildConnectionOpenInitMessage<CosmosChain>
    + CanBuildConnectionOpenTryMessage<CosmosChain>
    + CanBuildConnectionOpenAckMessage<CosmosChain>
    + CanBuildConnectionOpenConfirmMessage<CosmosChain>
    + CanBuildChannelOpenTryMessage<CosmosChain>
    + CanBuildChannelOpenAckMessage<CosmosChain>
    + CanBuildChannelOpenConfirmMessage<CosmosChain>
    + HasInitChannelOptionsType<CosmosChain>
    + HasConnectionOpenInitEvent<CosmosChain>
    + HasConnectionOpenTryEvent<CosmosChain>
{
}

impl CanUseSovereignChain for SovereignChain {}
