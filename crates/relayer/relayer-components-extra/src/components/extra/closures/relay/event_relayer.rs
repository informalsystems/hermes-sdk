use cgp::core::component::HasComponents;
use cgp::core::error::ErrorRaiser;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_relayer_components::relay::impls::packet_relayers::general::lock::LogSkipRelayLockedPacket;
use hermes_relayer_components::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use hermes_relayer_components::relay::traits::event_relayer::CanRelayEvent;
use hermes_relayer_components::relay::traits::packet::HasRelayPacketFields;
use hermes_relayer_components::relay::traits::packet_filter::PacketFilter;
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
        + HasRelayPacketFields
        + HasLogger
        + UseExtraAckPacketRelayer
        + UseExtraPacketRelayer
        + HasComponents<Components = Components>,
    SrcChain: UseExtraChainComponentsForEventRelayer<DstChain>,
    DstChain: UseExtraChainComponentsForEventRelayer<SrcChain>,
    Components: DelegatesToExtraRelayComponents
        + PacketFilter<Relay>
        + ErrorRaiser<Relay, SrcChain::Error>
        + ErrorRaiser<Relay, DstChain::Error>,
    Relay::Logger: for<'a> CanLog<LogSkipRelayLockedPacket<'a, Relay>>,
{
}
