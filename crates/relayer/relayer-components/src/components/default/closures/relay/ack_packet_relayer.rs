use cgp::core::component::HasComponents;
use cgp::core::error::{ErrorRaiser, HasErrorType};
use hermes_chain_components::traits::send_message::EmptyMessageResponse;
use hermes_chain_components::traits::types::ibc::HasIbcChainTypes;
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;
use hermes_chain_components::traits::types::timestamp::HasTimeoutType;
use hermes_chain_type_components::traits::fields::message_response_events::HasMessageResponseEvents;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;

use crate::chain::traits::message_builders::ack_packet::CanBuildAckPacketMessage;
use crate::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use crate::chain::traits::packet::fields::CanReadOutgoingPacketFields;
use crate::chain::traits::payload_builders::ack_packet::CanBuildAckPacketPayload;
use crate::chain::traits::payload_builders::update_client::CanBuildUpdateClientPayload;
use crate::chain::traits::queries::chain_status::CanQueryChainStatus;
use crate::chain::traits::queries::client_state::CanQueryClientState;
use crate::chain::traits::queries::consensus_state::CanQueryConsensusState;
use crate::chain::traits::queries::consensus_state_height::CanQueryConsensusStateHeight;
use crate::chain::traits::send_message::CanSendMessages;
use crate::chain::traits::types::chain_id::HasChainId;
use crate::chain::traits::types::client_state::HasClientStateFields;
use crate::chain::traits::types::consensus_state::HasConsensusStateType;
use crate::chain::traits::types::ibc::HasCounterpartyMessageHeight;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::components::default::relay::{DelegatesToDefaultRelayComponents, MainSink};
use crate::relay::impls::update_client::skip::LogSkipBuildUpdateClientMessage;
use crate::relay::impls::update_client::wait::LogWaitUpdateClientHeightStatus;
use crate::relay::traits::chains::{HasRelayChains, HasRelayClientIds};
use crate::relay::traits::ibc_message_sender::CanSendSingleIbcMessage;
use crate::relay::traits::packet_relayers::ack_packet::CanRelayAckPacket;
use crate::relay::traits::target::{
    HasDestinationTargetChainTypes, HasSourceTargetChainTypes, SourceTarget,
};

pub trait CanUseDefaultAckPacketRelayer: UseDefaultAckPacketRelayer {}

pub trait UseDefaultAckPacketRelayer: CanRelayAckPacket {}

impl<Relay, SrcChain, DstChain, Components, Logger> UseDefaultAckPacketRelayer for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasSourceTargetChainTypes
        + HasDestinationTargetChainTypes
        + HasRelayClientIds
        + CanSendSingleIbcMessage<MainSink, SourceTarget>
        + HasLogger<Logger = Logger>
        + HasComponents<Components = Components>,
    SrcChain: HasErrorType
        + HasChainId
        + CanSendMessages
        + HasMessageResponseEvents
        + CanQueryChainStatus
        + HasOutgoingPacketType<DstChain>
        + HasIbcChainTypes<DstChain>
        + CanReadOutgoingPacketFields<DstChain>
        + HasConsensusStateType<DstChain>
        + HasCounterpartyMessageHeight<DstChain>
        + CanReadOutgoingPacketFields<DstChain>
        + CanQueryClientState<DstChain>
        + CanQueryConsensusState<DstChain>
        + CanQueryConsensusStateHeight<DstChain>
        + CanBuildAckPacketMessage<DstChain>
        + CanBuildUpdateClientMessage<DstChain>,
    DstChain: HasErrorType
        + HasRuntime
        + HasChainId
        + CanQueryChainStatus
        + HasTimeoutType
        + HasIbcChainTypes<SrcChain>
        + HasClientStateFields<SrcChain>
        + HasConsensusStateType<SrcChain>
        + CanReadOutgoingPacketFields<SrcChain>
        + CanBuildAckPacketPayload<SrcChain>
        + CanBuildUpdateClientPayload<SrcChain>
        + HasWriteAckEvent<SrcChain>,
    SrcChain::Height: Clone,
    DstChain::Height: Clone,
    DstChain::Runtime: CanSleep,
    Logger: for<'a> CanLog<LogSkipBuildUpdateClientMessage<'a, Relay, SourceTarget>>
        + for<'a> CanLog<LogWaitUpdateClientHeightStatus<'a, Relay, SourceTarget>>,
    Components: DelegatesToDefaultRelayComponents
        + ErrorRaiser<Relay, EmptyMessageResponse>
        + ErrorRaiser<Relay, SrcChain::Error>
        + ErrorRaiser<Relay, DstChain::Error>,
{
}
