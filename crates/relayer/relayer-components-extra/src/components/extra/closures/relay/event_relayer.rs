use cgp_core::HasComponents;
use hermes_relayer_components::logger::traits::has_logger::{HasLogger, HasLoggerType};
use hermes_relayer_components::logger::traits::level::HasBaseLogLevels;
use hermes_relayer_components::relay::traits::chains::HasRelayChains;
use hermes_relayer_components::relay::traits::components::event_relayer::CanRelayEvent;
use hermes_relayer_components::relay::traits::components::packet_filter::PacketFilter;
use hermes_relayer_components::relay::traits::packet::HasRelayPacketFields;
use hermes_relayer_components::relay::traits::packet_lock::HasPacketLock;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};

use crate::components::extra::closures::chain::event_relayer::UseExtraChainComponentsForEventRelayer;
use crate::components::extra::closures::relay::ack_packet_relayer::UseExtraAckPacketRelayer;
use crate::components::extra::closures::relay::packet_relayer::UseExtraPacketRelayer;
use crate::components::extra::relay::DelegatesToExtraRelayComponents;

pub trait CanUseExtraEventRelayer: UseExtraEventRelayer {}

pub trait UseExtraEventRelayer:
    CanRelayEvent<SourceTarget> + CanRelayEvent<DestinationTarget>
{
}

impl<Relay, Components> UseExtraEventRelayer for Relay
where
    Relay: HasRelayChains
        + HasPacketLock
        + HasLogger
        + HasRelayPacketFields
        + UseExtraAckPacketRelayer
        + UseExtraPacketRelayer
        + HasComponents<Components = Components>,
    Relay::SrcChain: HasLoggerType<Logger = Relay::Logger>
        + UseExtraChainComponentsForEventRelayer<Relay::DstChain>,
    Relay::DstChain: UseExtraChainComponentsForEventRelayer<Relay::SrcChain>,
    Relay::Logger: HasBaseLogLevels,
    Components: DelegatesToExtraRelayComponents + PacketFilter<Relay>,
{
}
