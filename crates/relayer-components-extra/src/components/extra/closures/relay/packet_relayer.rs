use cgp_core::{HasComponents, HasErrorType};
use ibc_relayer_components::chain::traits::components::ack_packet_message_builder::CanBuildAckPacketMessage;
use ibc_relayer_components::chain::traits::components::ack_packet_payload_builder::CanBuildAckPacketPayload;
use ibc_relayer_components::chain::traits::components::client_state_querier::CanQueryClientState;
use ibc_relayer_components::chain::traits::components::consensus_state_height_querier::CanQueryConsensusStateHeight;
use ibc_relayer_components::chain::traits::components::packet_fields_reader::CanReadPacketFields;
use ibc_relayer_components::chain::traits::components::receive_packet_message_builder::CanBuildReceivePacketMessage;
use ibc_relayer_components::chain::traits::components::receive_packet_payload_builder::CanBuildReceivePacketPayload;
use ibc_relayer_components::chain::traits::components::received_packet_querier::CanQueryReceivedPacket;
use ibc_relayer_components::chain::traits::components::timeout_unordered_packet_message_builder::{
    CanBuildTimeoutUnorderedPacketMessage, CanBuildTimeoutUnorderedPacketPayload,
};
use ibc_relayer_components::chain::traits::components::update_client_message_builder::CanBuildUpdateClientMessage;
use ibc_relayer_components::chain::traits::components::update_client_payload_builder::CanBuildUpdateClientPayload;
use ibc_relayer_components::chain::traits::logs::packet::CanLogChainPacket;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainId;
use ibc_relayer_components::chain::traits::types::client_state::HasClientStateFields;
use ibc_relayer_components::chain::traits::types::consensus_state::HasConsensusStateType;
use ibc_relayer_components::chain::traits::types::height::CanIncrementHeight;
use ibc_relayer_components::chain::traits::types::ibc::HasCounterpartyMessageHeight;
use ibc_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use ibc_relayer_components::logger::traits::has_logger::{HasLogger, HasLoggerType};
use ibc_relayer_components::logger::traits::level::HasBaseLogLevels;
use ibc_relayer_components::relay::traits::chains::HasRelayChains;
use ibc_relayer_components::relay::traits::components::packet_filter::PacketFilter;
use ibc_relayer_components::relay::traits::components::packet_relayer::CanRelayPacket;
use ibc_relayer_components::relay::traits::packet_lock::HasPacketLock;
use ibc_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_relayer_components::runtime::traits::sleep::CanSleep;

use crate::batch::traits::channel::HasMessageBatchSender;
use crate::components::extra::closures::chain::UseExtraChainComponents;
use crate::components::extra::relay::ExtraRelayComponents;
use crate::relay::components::packet_relayers::retry::SupportsPacketRetry;
use crate::runtime::traits::channel::CanUseChannels;
use crate::runtime::traits::channel_once::{CanCreateChannelsOnce, CanUseChannelsOnce};

pub trait CanUseExtraPacketRelayer: UseExtraPacketRelayer {}

pub trait UseExtraPacketRelayer: CanRelayPacket {}

impl<Relay, SrcChain, DstChain, BaseRelayComponents> UseExtraPacketRelayer for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasLogger
        + HasPacketLock
        + SupportsPacketRetry
        + HasMessageBatchSender<SourceTarget>
        + HasMessageBatchSender<DestinationTarget>
        + HasComponents<Components = ExtraRelayComponents<BaseRelayComponents>>,
    SrcChain: HasErrorType
        + HasRuntime
        + HasChainId
        + CanIncrementHeight
        + HasLoggerType<Logger = Relay::Logger>
        + HasClientStateFields<DstChain>
        + HasConsensusStateType<DstChain>
        + HasCounterpartyMessageHeight<DstChain>
        + CanReadPacketFields<DstChain, OutgoingPacket = Relay::Packet>
        + CanLogChainPacket<DstChain>
        + CanQueryClientState<DstChain>
        + CanQueryConsensusStateHeight<DstChain>
        + CanBuildReceivePacketPayload<DstChain>
        + CanBuildUpdateClientPayload<DstChain>
        + CanBuildAckPacketMessage<DstChain>
        + CanBuildUpdateClientMessage<DstChain>
        + CanBuildTimeoutUnorderedPacketMessage<DstChain>
        + UseExtraChainComponents<DstChain>,
    DstChain: HasErrorType
        + HasRuntime
        + HasChainId
        + CanIncrementHeight
        + HasClientStateFields<SrcChain>
        + HasConsensusStateType<SrcChain>
        + HasCounterpartyMessageHeight<SrcChain>
        + HasWriteAckEvent<SrcChain>
        + CanReadPacketFields<SrcChain, IncomingPacket = Relay::Packet>
        + CanQueryClientState<SrcChain>
        + CanQueryReceivedPacket<SrcChain>
        + CanQueryConsensusStateHeight<SrcChain>
        + CanBuildAckPacketPayload<SrcChain>
        + CanBuildUpdateClientPayload<SrcChain>
        + CanBuildTimeoutUnorderedPacketPayload<SrcChain>
        + CanBuildUpdateClientMessage<SrcChain>
        + CanBuildReceivePacketMessage<SrcChain>
        + UseExtraChainComponents<SrcChain>,
    SrcChain::Height: Clone,
    DstChain::Height: Clone,
    SrcChain::Runtime: CanSleep + CanCreateChannelsOnce + CanUseChannels + CanUseChannelsOnce,
    DstChain::Runtime: CanSleep + CanCreateChannelsOnce + CanUseChannels + CanUseChannelsOnce,
    Relay::Logger: HasBaseLogLevels,
    BaseRelayComponents: PacketFilter<Relay>,
{
}
