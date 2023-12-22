use cgp_core::{HasComponents, HasErrorType};

use crate::chain::traits::components::ack_packet_message_builder::CanBuildAckPacketMessage;
use crate::chain::traits::components::ack_packet_payload_builder::CanBuildAckPacketPayload;
use crate::chain::traits::components::chain_status_querier::CanQueryChainStatus;
use crate::chain::traits::components::client_state_querier::CanQueryClientState;
use crate::chain::traits::components::consensus_state_height_querier::CanQueryConsensusStateHeight;
use crate::chain::traits::components::consensus_state_querier::CanQueryConsensusState;
use crate::chain::traits::components::message_sender::CanSendMessages;
use crate::chain::traits::components::packet_fields_reader::CanReadPacketFields;
use crate::chain::traits::components::update_client_message_builder::CanBuildUpdateClientMessage;
use crate::chain::traits::components::update_client_payload_builder::CanBuildUpdateClientPayload;
use crate::chain::traits::types::chain_id::HasChainId;
use crate::chain::traits::types::client_state::HasClientStateFields;
use crate::chain::traits::types::consensus_state::HasConsensusStateType;
use crate::chain::traits::types::height::CanIncrementHeight;
use crate::chain::traits::types::ibc::HasCounterpartyMessageHeight;
use crate::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use crate::components::default::relay::DelegatesToDefaultRelayComponents;
use crate::logger::traits::has_logger::HasLogger;
use crate::logger::traits::level::HasBaseLogLevels;
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::components::packet_relayers::ack_packet::CanRelayAckPacket;
use crate::runtime::traits::runtime::HasRuntime;
use crate::runtime::traits::sleep::CanSleep;

pub trait CanUseDefaultAckPacketRelayer: UseDefaultAckPacketRelayer
where
    Self::DstChain: HasWriteAckEvent<Self::SrcChain>,
{
}

pub trait UseDefaultAckPacketRelayer: CanRelayAckPacket
where
    Self::DstChain: HasWriteAckEvent<Self::SrcChain>,
{
}

impl<Relay, SrcChain, DstChain, Components> UseDefaultAckPacketRelayer for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasLogger
        + HasComponents<Components = Components>,
    SrcChain: HasErrorType
        + HasChainId
        + CanSendMessages
        + HasConsensusStateType<DstChain>
        + HasCounterpartyMessageHeight<DstChain>
        + CanReadPacketFields<DstChain, OutgoingPacket = Relay::Packet>
        + CanQueryClientState<DstChain>
        + CanQueryConsensusState<DstChain>
        + CanQueryConsensusStateHeight<DstChain>
        + CanBuildAckPacketMessage<DstChain>
        + CanBuildUpdateClientMessage<DstChain>,
    DstChain: HasErrorType
        + HasRuntime
        + HasChainId
        + CanIncrementHeight
        + CanQueryChainStatus
        + HasClientStateFields<SrcChain>
        + HasConsensusStateType<SrcChain>
        + CanReadPacketFields<SrcChain, IncomingPacket = Relay::Packet>
        + CanBuildAckPacketPayload<SrcChain>
        + CanBuildUpdateClientPayload<SrcChain>,
    SrcChain::Height: Clone,
    DstChain::Height: Clone,
    DstChain::Runtime: CanSleep,
    Relay::Logger: HasBaseLogLevels,
    Components: DelegatesToDefaultRelayComponents,
{
}
