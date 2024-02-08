use cgp_core::{ErrorRaiser, HasComponents, HasErrorType};

use crate::chain::traits::logs::packet::CanLogChainPacket;
use crate::chain::traits::message_builders::ack_packet::CanBuildAckPacketMessage;
use crate::chain::traits::message_builders::receive_packet::CanBuildReceivePacketMessage;
use crate::chain::traits::message_builders::timeout_unordered_packet::{
    CanBuildTimeoutUnorderedPacketMessage, CanBuildTimeoutUnorderedPacketPayload,
};
use crate::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use crate::chain::traits::packet::fields::CanReadPacketFields;
use crate::chain::traits::payload_builders::ack_packet::CanBuildAckPacketPayload;
use crate::chain::traits::payload_builders::receive_packet::CanBuildReceivePacketPayload;
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
use crate::chain::traits::types::height::CanIncrementHeight;
use crate::chain::traits::types::ibc::HasCounterpartyMessageHeight;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::components::default::relay::DelegatesToDefaultRelayComponents;
use crate::logger::traits::has_logger::{HasLogger, HasLoggerType};
use crate::logger::traits::level::HasBaseLogLevels;
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::components::packet_filter::PacketFilter;
use crate::relay::traits::components::packet_relayer::CanRelayPacket;
use crate::relay::traits::packet_lock::HasPacketLock;
use crate::runtime::traits::runtime::HasRuntime;
use crate::runtime::traits::sleep::CanSleep;

pub trait CanUseDefaultPacketRelayer: UseDefaultPacketRelayer {}

pub trait UseDefaultPacketRelayer: CanRelayPacket {}

impl<Relay, SrcChain, DstChain, Components> UseDefaultPacketRelayer for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasLogger
        + HasPacketLock
        + HasComponents<Components = Components>,
    SrcChain: HasErrorType
        + HasRuntime
        + HasChainId
        + CanSendMessages
        + CanIncrementHeight
        + CanQueryChainStatus
        + HasLoggerType<Logger = Relay::Logger>
        + HasClientStateFields<DstChain>
        + HasConsensusStateType<DstChain>
        + HasCounterpartyMessageHeight<DstChain>
        + CanReadPacketFields<DstChain, OutgoingPacket = Relay::Packet>
        + CanLogChainPacket<DstChain>
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
        + CanIncrementHeight
        + CanQueryChainStatus
        + HasClientStateFields<SrcChain>
        + HasConsensusStateType<SrcChain>
        + HasCounterpartyMessageHeight<SrcChain>
        + HasWriteAckEvent<SrcChain>
        + CanReadPacketFields<SrcChain, IncomingPacket = Relay::Packet>
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
    Relay::Logger: HasBaseLogLevels,
    Components: DelegatesToDefaultRelayComponents
        + PacketFilter<Relay>
        + ErrorRaiser<Relay, SrcChain::Error>
        + ErrorRaiser<Relay, DstChain::Error>,
{
}
