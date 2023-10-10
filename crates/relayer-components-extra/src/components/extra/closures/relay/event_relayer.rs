use cgp_core::HasComponents;
use ibc_relayer_components::logger::traits::has_logger::{HasLogger, HasLoggerType};
use ibc_relayer_components::logger::traits::level::HasBaseLogLevels;
use ibc_relayer_components::relay::traits::chains::HasRelayChains;
use ibc_relayer_components::relay::traits::components::event_relayer::CanRelayEvent;
use ibc_relayer_components::relay::traits::components::packet_filter::PacketFilter;
use ibc_relayer_components::relay::traits::packet::HasRelayPacketFields;
use ibc_relayer_components::relay::traits::packet_lock::HasPacketLock;
use ibc_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};

use crate::components::extra::closures::chain::UseExtraChainComponents;
use crate::components::extra::closures::relay::ack_packet_relayer::UseExtraAckPacketRelayer;
use crate::components::extra::closures::relay::packet_relayer::UseExtraPacketRelayer;
use crate::components::extra::relay::ExtraRelayComponents;

pub trait CanUseExtraEventRelayer: UseExtraEventRelayer {}

pub trait UseExtraEventRelayer:
    CanRelayEvent<SourceTarget> + CanRelayEvent<DestinationTarget>
{
}

impl<Relay, BaseRelayComponents> UseExtraEventRelayer for Relay
where
    Relay: HasRelayChains
        + HasPacketLock
        + HasLogger
        + HasRelayPacketFields
        + UseExtraAckPacketRelayer
        + UseExtraPacketRelayer
        + HasComponents<Components = ExtraRelayComponents<BaseRelayComponents>>,
    Relay::SrcChain:
        HasLoggerType<Logger = Relay::Logger> + UseExtraChainComponents<Relay::DstChain>,
    Relay::DstChain: UseExtraChainComponents<Relay::SrcChain>,
    Relay::Logger: HasBaseLogLevels,
    BaseRelayComponents: PacketFilter<Relay>,
{
}
