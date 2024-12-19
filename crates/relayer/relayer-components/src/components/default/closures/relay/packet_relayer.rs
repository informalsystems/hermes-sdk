use cgp::core::component::HasComponents;
use cgp::core::error::{ErrorRaiser, HasErrorType};
use hermes_chain_components::traits::send_message::EmptyMessageResponse;
use hermes_chain_components::traits::types::ibc::HasIbcChainTypes;
use hermes_chain_type_components::traits::fields::message_response_events::HasMessageResponseEvents;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;

use crate::chain::traits::message_builders::ack_packet::CanBuildAckPacketMessage;
use crate::chain::traits::message_builders::receive_packet::CanBuildReceivePacketMessage;
use crate::chain::traits::message_builders::timeout_unordered_packet::CanBuildTimeoutUnorderedPacketMessage;
use crate::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use crate::chain::traits::packet::fields::CanReadPacketFields;
use crate::chain::traits::payload_builders::ack_packet::CanBuildAckPacketPayload;
use crate::chain::traits::payload_builders::receive_packet::CanBuildReceivePacketPayload;
use crate::chain::traits::payload_builders::timeout_unordered_packet::CanBuildTimeoutUnorderedPacketPayload;
use crate::chain::traits::payload_builders::update_client::CanBuildUpdateClientPayload;
use crate::chain::traits::queries::chain_status::CanQueryChainStatus;
use crate::chain::traits::queries::client_state::CanQueryClientState;
use crate::chain::traits::queries::consensus_state::CanQueryConsensusState;
use crate::chain::traits::queries::consensus_state_height::CanQueryConsensusStateHeight;
use crate::chain::traits::queries::packet_is_received::CanQueryPacketIsReceived;
use crate::chain::traits::send_message::CanSendMessages;
use crate::chain::traits::types::chain_id::HasChainId;
use crate::chain::traits::types::client_state::HasClientStateFields;
use crate::chain::traits::types::consensus_state::HasConsensusStateType;
use crate::chain::traits::types::ibc::HasCounterpartyMessageHeight;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::components::default::relay::{DelegatesToDefaultRelayPreset, MainSink};
use crate::relay::impls::packet_relayers::general::full_relay::LogRelayPacketAction;
use crate::relay::impls::packet_relayers::general::lock::LogSkipRelayLockedPacket;
use crate::relay::impls::packet_relayers::general::log::LogRelayPacketStatus;
use crate::relay::impls::update_client::skip::LogSkipBuildUpdateClientMessage;
use crate::relay::impls::update_client::wait::LogWaitUpdateClientHeightStatus;
use crate::relay::traits::chains::{HasRelayChains, HasRelayClientIds};
use crate::relay::traits::ibc_message_sender::CanSendSingleIbcMessage;
use crate::relay::traits::packet_filter::RelayPacketFilter;
use crate::relay::traits::packet_lock::HasPacketLock;
use crate::relay::traits::packet_relayer::CanRelayPacket;
use crate::relay::traits::packet_relayers::receive_packet::CanRelayReceivePacket;
use crate::relay::traits::packet_relayers::timeout_unordered_packet::CanRelayTimeoutUnorderedPacket;
use crate::relay::traits::target::{
    DestinationTarget, HasDestinationTargetChainTypes, HasSourceTargetChainTypes, SourceTarget,
};

pub trait CanUseDefaultPacketRelayer: UseDefaultPacketRelayer {}

pub trait UseDefaultPacketRelayer: CanRelayPacket {}

impl<Relay, SrcChain, DstChain, Components, Logger> UseDefaultPacketRelayer for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasSourceTargetChainTypes
        + HasDestinationTargetChainTypes
        + HasRelayClientIds
        + CanRelayReceivePacket
        + CanRelayTimeoutUnorderedPacket
        + CanSendSingleIbcMessage<MainSink, SourceTarget>
        + HasLogger<Logger = Logger>
        + HasPacketLock
        + HasComponents<Components = Components>,
    SrcChain: HasErrorType
        + HasRuntime
        + HasChainId
        + CanSendMessages
        + HasMessageResponseEvents
        + CanQueryChainStatus
        + HasIbcChainTypes<DstChain>
        + HasClientStateFields<DstChain>
        + HasConsensusStateType<DstChain>
        + HasCounterpartyMessageHeight<DstChain>
        + CanReadPacketFields<DstChain>
        + CanQueryClientState<DstChain>
        + CanQueryConsensusState<DstChain>
        + CanQueryConsensusStateHeight<DstChain>
        + CanBuildReceivePacketPayload<DstChain>
        + CanBuildUpdateClientPayload<DstChain>
        + CanBuildAckPacketMessage<DstChain>
        + CanBuildUpdateClientMessage<DstChain>
        + CanBuildTimeoutUnorderedPacketMessage<DstChain>,
    DstChain: HasErrorType
        + HasRuntime
        + HasChainId
        + CanSendMessages
        + HasMessageResponseEvents
        + CanQueryChainStatus
        + HasClientStateFields<SrcChain>
        + HasConsensusStateType<SrcChain>
        + HasCounterpartyMessageHeight<SrcChain>
        + HasWriteAckEvent<SrcChain>
        + CanReadPacketFields<SrcChain>
        + CanQueryClientState<SrcChain>
        + CanQueryPacketIsReceived<SrcChain>
        + CanQueryConsensusState<SrcChain>
        + CanQueryConsensusStateHeight<SrcChain>
        + CanBuildAckPacketPayload<SrcChain>
        + CanBuildUpdateClientPayload<SrcChain>
        + CanBuildTimeoutUnorderedPacketPayload<SrcChain>
        + CanBuildUpdateClientMessage<SrcChain>
        + CanBuildReceivePacketMessage<SrcChain>,
    SrcChain::Height: Clone,
    DstChain::Height: Clone,
    SrcChain::Runtime: CanSleep,
    DstChain::Runtime: CanSleep,
    Logger: for<'a> CanLog<LogSkipRelayLockedPacket<'a, Relay>>
        + for<'a> CanLog<LogRelayPacketAction<'a, Relay>>
        + for<'a> CanLog<LogRelayPacketStatus<'a, Relay>>
        + for<'a> CanLog<LogSkipBuildUpdateClientMessage<'a, Relay, SourceTarget>>
        + for<'a> CanLog<LogSkipBuildUpdateClientMessage<'a, Relay, DestinationTarget>>
        + for<'a> CanLog<LogWaitUpdateClientHeightStatus<'a, Relay, SourceTarget>>
        + for<'a> CanLog<LogWaitUpdateClientHeightStatus<'a, Relay, DestinationTarget>>,
    Components: DelegatesToDefaultRelayPreset
        + RelayPacketFilter<Relay>
        + ErrorRaiser<Relay, EmptyMessageResponse>
        + ErrorRaiser<Relay, SrcChain::Error>
        + ErrorRaiser<Relay, DstChain::Error>,
{
}
