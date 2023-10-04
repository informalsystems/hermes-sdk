use cgp_core::{Async, HasComponents, HasErrorType};
use ibc_relayer_components::chain::traits::components::ack_packet_message_builder::CanBuildAckPacketMessage;
use ibc_relayer_components::chain::traits::components::ack_packet_payload_builder::CanBuildAckPacketPayload;
use ibc_relayer_components::chain::traits::components::chain_status_querier::CanQueryChainStatus;
use ibc_relayer_components::chain::traits::components::client_state_querier::CanQueryClientState;
use ibc_relayer_components::chain::traits::components::consensus_state_height_querier::CanQueryConsensusStateHeight;
use ibc_relayer_components::chain::traits::components::consensus_state_querier::CanQueryConsensusState;
use ibc_relayer_components::chain::traits::components::message_sender::CanSendMessages;
use ibc_relayer_components::chain::traits::components::packet_fields_reader::CanReadPacketFields;
use ibc_relayer_components::chain::traits::components::update_client_message_builder::CanBuildUpdateClientMessage;
use ibc_relayer_components::chain::traits::components::update_client_payload_builder::CanBuildUpdateClientPayload;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainId;
use ibc_relayer_components::chain::traits::types::client_state::HasClientStateFields;
use ibc_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use ibc_relayer_components::chain::traits::types::height::CanIncrementHeight;
use ibc_relayer_components::chain::traits::types::ibc::HasCounterpartyMessageHeight;
use ibc_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use ibc_relayer_components::logger::traits::has_logger::HasLogger;
use ibc_relayer_components::logger::traits::level::HasBaseLogLevels;
use ibc_relayer_components::relay::traits::chains::HasRelayChains;
use ibc_relayer_components::relay::traits::components::packet_relayers::ack_packet::CanRelayAckPacket;
use ibc_relayer_components::relay::traits::target::SourceTarget;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_relayer_components::runtime::traits::sleep::CanSleep;

use crate::batch::traits::channel::HasMessageBatchSender;
use crate::components::extra::relay::ExtraRelayComponents;
use crate::runtime::traits::channel::CanUseChannels;
use crate::runtime::traits::channel_once::{CanCreateChannelsOnce, CanUseChannelsOnce};

pub trait CanUseExtraAckPacketRelayer: UseExtraAckPacketRelayer
where
    Self::DstChain: HasWriteAckEvent<Self::SrcChain>,
{
}

pub trait UseExtraAckPacketRelayer: CanRelayAckPacket
where
    Self::DstChain: HasWriteAckEvent<Self::SrcChain>,
{
}

impl<Relay, SrcChain, DstChain, BaseRelayComponents> UseExtraAckPacketRelayer for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasLogger
        + HasMessageBatchSender<SourceTarget>
        + HasComponents<Components = ExtraRelayComponents<BaseRelayComponents>>,
    SrcChain: HasErrorType
        + HasRuntime
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
    SrcChain::Runtime: CanCreateChannelsOnce + CanUseChannels + CanUseChannelsOnce,
    DstChain::Runtime: CanSleep,
    Relay::Logger: HasBaseLogLevels,
    BaseRelayComponents: Async,
{
}
