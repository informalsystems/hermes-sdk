use cgp_core::{ErrorRaiser, HasComponents, HasErrorType};
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;

use crate::chain::traits::message_builders::ack_packet::CanBuildAckPacketMessage;
use crate::chain::traits::message_builders::update_client::CanBuildUpdateClientMessage;
use crate::chain::traits::packet::fields::CanReadPacketFields;
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
use crate::components::default::relay::DelegatesToDefaultRelayComponents;
use crate::relay::impls::update_client::skip::LogSkipBuildUpdateClientMessage;
use crate::relay::impls::update_client::wait::LogWaitUpdateClientHeightStatus;
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::packet_relayers::ack_packet::CanRelayAckPacket;
use crate::relay::traits::target::SourceTarget;

pub trait CanUseDefaultAckPacketRelayer: UseDefaultAckPacketRelayer {}

pub trait UseDefaultAckPacketRelayer: CanRelayAckPacket {}

impl<Relay, SrcChain, DstChain, Components, Logger> UseDefaultAckPacketRelayer for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasLogger<Logger = Logger>
        + HasComponents<Components = Components>,
    SrcChain: HasErrorType
        + HasChainId
        + CanSendMessages
        + CanQueryChainStatus
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
        + CanQueryChainStatus
        + HasClientStateFields<SrcChain>
        + HasConsensusStateType<SrcChain>
        + CanReadPacketFields<SrcChain, IncomingPacket = Relay::Packet>
        + CanBuildAckPacketPayload<SrcChain>
        + CanBuildUpdateClientPayload<SrcChain>
        + HasWriteAckEvent<SrcChain>,
    SrcChain::Height: Clone,
    DstChain::Height: Clone,
    DstChain::Runtime: CanSleep,
    Logger: for<'a> CanLog<LogSkipBuildUpdateClientMessage<'a, Relay, SourceTarget>>
        + for<'a> CanLog<LogWaitUpdateClientHeightStatus<'a, Relay, SourceTarget>>,
    Components: DelegatesToDefaultRelayComponents
        + ErrorRaiser<Relay, SrcChain::Error>
        + ErrorRaiser<Relay, DstChain::Error>,
{
}
