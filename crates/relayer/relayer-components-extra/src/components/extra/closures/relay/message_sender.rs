use cgp_core::{CanRaiseError, ErrorRaiser, HasComponents};
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::log::traits::has_logger::HasLogger;
use hermes_relayer_components::log::traits::logger::CanLog;
use hermes_relayer_components::logger::traits::level::HasBaseLogLevels;
use hermes_relayer_components::relay::impls::update_client::skip::LogSkipBuildUpdateClientMessage;
use hermes_relayer_components::relay::impls::update_client::wait::LogWaitUpdateClientHeightStatus;
use hermes_relayer_components::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use hermes_relayer_components::relay::traits::ibc_message_sender::{CanSendIbcMessages, MainSink};
use hermes_relayer_components::relay::traits::packet_filter::PacketFilter;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components::runtime::traits::sleep::CanSleep;
use hermes_relayer_components::runtime::types::aliases::ErrorOf;

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
    + CanRaiseRelayChainErrors
{
}

impl<Relay, SrcChain, DstChain, Components, OldLogger, Logger> UseExtraIbcMessageSender for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + hermes_relayer_components::logger::traits::has_logger::HasLogger<Logger = OldLogger>
        + HasLogger<Logger = Logger>
        + HasMessageBatchSender<SourceTarget>
        + HasMessageBatchSender<DestinationTarget>
        + HasComponents<Components = Components>,
    SrcChain: hermes_relayer_components::logger::traits::has_logger::HasLoggerType<Logger = OldLogger>
        + HasIbcPacketTypes<DstChain, OutgoingPacket = Relay::Packet>
        + UseExtraChainComponentsForIbcMessageSender<DstChain>
        + CanRaiseError<ErrorOf<SrcChain::Runtime>>,
    DstChain: HasIbcPacketTypes<SrcChain, IncomingPacket = Relay::Packet>
        + UseExtraChainComponentsForIbcMessageSender<SrcChain>
        + CanRaiseError<ErrorOf<DstChain::Runtime>>,
    SrcChain::Height: Clone,
    DstChain::Height: Clone,
    SrcChain::Runtime: CanSleep + CanCreateChannelsOnce + CanUseChannels + CanUseChannelsOnce,
    DstChain::Runtime: CanSleep + CanCreateChannelsOnce + CanUseChannels + CanUseChannelsOnce,
    OldLogger: HasBaseLogLevels,
    Logger: for<'a> CanLog<LogSkipBuildUpdateClientMessage<'a, Relay, SourceTarget>>
        + for<'a> CanLog<LogSkipBuildUpdateClientMessage<'a, Relay, DestinationTarget>>
        + for<'a> CanLog<LogWaitUpdateClientHeightStatus<'a, Relay, SourceTarget>>
        + for<'a> CanLog<LogWaitUpdateClientHeightStatus<'a, Relay, DestinationTarget>>,
    Components: DelegatesToExtraRelayComponents
        + PacketFilter<Relay>
        + ErrorRaiser<Relay, SrcChain::Error>
        + ErrorRaiser<Relay, DstChain::Error>,
{
}
