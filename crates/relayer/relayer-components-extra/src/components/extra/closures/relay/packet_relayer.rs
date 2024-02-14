use cgp_core::HasComponents;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::logger::traits::has_logger::{HasLogger, HasLoggerType};
use hermes_relayer_components::logger::traits::level::HasBaseLogLevels;
use hermes_relayer_components::relay::traits::chains::HasRelayChains;
use hermes_relayer_components::relay::traits::packet_filter::PacketFilter;
use hermes_relayer_components::relay::traits::packet_lock::HasPacketLock;
use hermes_relayer_components::relay::traits::packet_relayer::CanRelayPacket;

use crate::components::extra::closures::chain::packet_relayer::UseExtraChainComponentsForPacketRelayer;
use crate::components::extra::closures::relay::message_sender::UseExtraIbcMessageSender;
use crate::components::extra::relay::DelegatesToExtraRelayComponents;
use crate::relay::components::packet_relayers::retry::SupportsPacketRetry;

pub trait CanUseExtraPacketRelayer: UseExtraPacketRelayer {}

pub trait UseExtraPacketRelayer: CanRelayPacket {}

impl<Relay, SrcChain, DstChain, Components> UseExtraPacketRelayer for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasLogger
        + HasPacketLock
        + SupportsPacketRetry
        + UseExtraIbcMessageSender
        + HasComponents<Components = Components>,
    SrcChain: HasLoggerType<Logger = Relay::Logger>
        + HasIbcPacketTypes<DstChain, OutgoingPacket = Relay::Packet>
        + UseExtraChainComponentsForPacketRelayer<DstChain>,
    DstChain: HasIbcPacketTypes<SrcChain, IncomingPacket = Relay::Packet>
        + UseExtraChainComponentsForPacketRelayer<SrcChain>,
    Relay::Logger: HasBaseLogLevels,
    Components: DelegatesToExtraRelayComponents + PacketFilter<Relay>,
{
}
