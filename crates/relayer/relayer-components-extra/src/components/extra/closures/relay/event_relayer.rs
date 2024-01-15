use cgp_core::{ErrorRaiser, HasComponents};
use hermes_relayer_components::logger::traits::has_logger::{HasLogger, HasLoggerType};
use hermes_relayer_components::logger::traits::level::HasBaseLogLevels;
use hermes_relayer_components::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
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
    CanRelayEvent<SourceTarget> + CanRelayEvent<DestinationTarget> + CanRaiseRelayChainErrors
{
}

impl<Relay, SrcChain, DstChain, Components> UseExtraEventRelayer for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasPacketLock
        + HasLogger
        + HasRelayPacketFields
        + UseExtraAckPacketRelayer
        + UseExtraPacketRelayer
        + HasComponents<Components = Components>,
    SrcChain:
        HasLoggerType<Logger = Relay::Logger> + UseExtraChainComponentsForEventRelayer<DstChain>,
    DstChain: UseExtraChainComponentsForEventRelayer<SrcChain>,
    Relay::Logger: HasBaseLogLevels,
    Components: DelegatesToExtraRelayComponents
        + PacketFilter<Relay>
        + ErrorRaiser<Relay, SrcChain::Error>
        + ErrorRaiser<Relay, DstChain::Error>,
{
}
