use cgp::core::component::HasComponents;
use cgp::core::error::{CanRaiseError, ErrorOf, ErrorRaiser};
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use hermes_relayer_components::relay::impls::update_client::skip::LogSkipBuildUpdateClientMessage;
use hermes_relayer_components::relay::impls::update_client::wait::LogWaitUpdateClientHeightStatus;
use hermes_relayer_components::relay::traits::chains::{
    CanRaiseRelayChainErrors, HasRelayChains, HasRelayClientIds,
};
use hermes_relayer_components::relay::traits::ibc_message_sender::{CanSendIbcMessages, MainSink};
use hermes_relayer_components::relay::traits::packet_filter::PacketFilter;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_runtime_components::traits::channel::CanUseChannels;
use hermes_runtime_components::traits::channel_once::{CanCreateChannelsOnce, CanUseChannelsOnce};
use hermes_runtime_components::traits::sleep::CanSleep;

use crate::batch::traits::channel::HasMessageBatchSender;
use crate::batch::types::sink::BatchWorkerSink;
use crate::components::extra::closures::chain::message_sender::UseExtraChainComponentsForIbcMessageSender;
use crate::components::extra::relay::DelegatesToExtraRelayComponents;

pub trait UseExtraIbcMessageSender:
    HasRelayClientIds
    + CanSendIbcMessages<MainSink, SourceTarget>
    + CanSendIbcMessages<MainSink, DestinationTarget>
    + CanSendIbcMessages<BatchWorkerSink, SourceTarget>
    + CanSendIbcMessages<BatchWorkerSink, DestinationTarget>
    + CanRaiseRelayChainErrors
{
}

impl<Relay, SrcChain, DstChain, Components, Logger> UseExtraIbcMessageSender for Relay
where
    Relay: HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasRelayClientIds
        + HasLogger<Logger = Logger>
        + HasMessageBatchSender<SourceTarget>
        + HasMessageBatchSender<DestinationTarget>
        + HasComponents<Components = Components>,
    SrcChain: HasIbcChainTypes<DstChain>
        + HasOutgoingPacketType<DstChain>
        + UseExtraChainComponentsForIbcMessageSender<DstChain>
        + CanRaiseError<ErrorOf<SrcChain::Runtime>>,
    DstChain: HasIbcChainTypes<SrcChain>
        + UseExtraChainComponentsForIbcMessageSender<SrcChain>
        + CanRaiseError<ErrorOf<DstChain::Runtime>>,
    SrcChain::Height: Clone,
    DstChain::Height: Clone,
    SrcChain::Runtime: CanSleep + CanCreateChannelsOnce + CanUseChannels + CanUseChannelsOnce,
    DstChain::Runtime: CanSleep + CanCreateChannelsOnce + CanUseChannels + CanUseChannelsOnce,
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
