use cgp_core::HasComponents;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::logger::traits::has_logger::{HasLogger, HasLoggerType};
use hermes_relayer_components::logger::traits::level::HasBaseLogLevels;
use hermes_relayer_components::relay::traits::chains::HasRelayChains;
use hermes_relayer_components::relay::traits::components::ibc_message_sender::{
    CanSendIbcMessages, MainSink,
};
use hermes_relayer_components::relay::traits::components::packet_filter::PacketFilter;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components::runtime::traits::sleep::CanSleep;

use crate::batch::traits::channel::HasMessageBatchSender;
use crate::batch::types::sink::BatchWorkerSink;
use crate::components::extra::closures::chain::message_sender::UseExtraChainComponentsForIbcMessageSender;
use crate::components::extra::relay::DelegatesToExtraRelayComponents;
use crate::runtime::traits::channel::CanUseChannels;
use crate::runtime::traits::channel_once::{CanCreateChannelsOnce, CanUseChannelsOnce};

pub trait UseExtraIbcMessageSender:
    CanSendIbcMessages<MainSink, SourceTarget>
    + CanSendIbcMessages<MainSink, DestinationTarget>
    + CanSendIbcMessages<BatchWorkerSink, SourceTarget>
    + CanSendIbcMessages<BatchWorkerSink, DestinationTarget>
{
}

impl<Relay, SrcChain, DstChain, Components> UseExtraIbcMessageSender for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasLogger
        + HasMessageBatchSender<SourceTarget>
        + HasMessageBatchSender<DestinationTarget>
        + HasComponents<Components = Components>,
    SrcChain: HasLoggerType<Logger = Relay::Logger>
        + HasIbcPacketTypes<DstChain, OutgoingPacket = Relay::Packet>
        + UseExtraChainComponentsForIbcMessageSender<DstChain>,
    DstChain: HasIbcPacketTypes<SrcChain, IncomingPacket = Relay::Packet>
        + UseExtraChainComponentsForIbcMessageSender<SrcChain>,
    SrcChain::Height: Clone,
    DstChain::Height: Clone,
    SrcChain::Runtime: CanSleep + CanCreateChannelsOnce + CanUseChannels + CanUseChannelsOnce,
    DstChain::Runtime: CanSleep + CanCreateChannelsOnce + CanUseChannels + CanUseChannelsOnce,
    Relay::Logger: HasBaseLogLevels,
    Components: DelegatesToExtraRelayComponents + PacketFilter<Relay>,
{
}
