use cgp_core::HasComponents;
use ibc_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use ibc_relayer_components::logger::traits::has_logger::{HasLogger, HasLoggerType};
use ibc_relayer_components::logger::traits::level::HasBaseLogLevels;
use ibc_relayer_components::relay::traits::chains::HasRelayChains;
use ibc_relayer_components::relay::traits::components::packet_filter::PacketFilter;
use ibc_relayer_components::relay::traits::components::packet_relayer::CanRelayPacket;
use ibc_relayer_components::relay::traits::packet_lock::HasPacketLock;
use ibc_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use ibc_relayer_components::runtime::traits::sleep::CanSleep;

use crate::batch::traits::channel::HasMessageBatchSender;
use crate::components::extra::closures::chain::UseExtraChainComponents;
use crate::components::extra::relay::ExtraRelayComponents;
use crate::relay::components::packet_relayers::retry::SupportsPacketRetry;
use crate::runtime::traits::channel::CanUseChannels;
use crate::runtime::traits::channel_once::{CanCreateChannelsOnce, CanUseChannelsOnce};

pub trait CanUseExtraPacketRelayer: UseExtraPacketRelayer {}

pub trait UseExtraPacketRelayer: CanRelayPacket {}

impl<Relay, SrcChain, DstChain, RelayComponents> UseExtraPacketRelayer for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasLogger
        + HasPacketLock
        + SupportsPacketRetry
        + HasMessageBatchSender<SourceTarget>
        + HasMessageBatchSender<DestinationTarget>
        + HasComponents<Components = ExtraRelayComponents<RelayComponents>>,
    SrcChain: HasLoggerType<Logger = Relay::Logger>
        + HasIbcPacketTypes<DstChain, OutgoingPacket = Relay::Packet>
        + UseExtraChainComponents<DstChain>,
    DstChain: HasIbcPacketTypes<SrcChain, IncomingPacket = Relay::Packet>
        + UseExtraChainComponents<SrcChain>,
    SrcChain::Height: Clone,
    DstChain::Height: Clone,
    SrcChain::Runtime: CanSleep + CanCreateChannelsOnce + CanUseChannels + CanUseChannelsOnce,
    DstChain::Runtime: CanSleep + CanCreateChannelsOnce + CanUseChannels + CanUseChannelsOnce,
    Relay::Logger: HasBaseLogLevels,
    RelayComponents: PacketFilter<Relay>,
{
}
