use cgp::core::component::HasComponents;
use cgp::core::error::{CanRaiseError, ErrorOf, ErrorRaiser};
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_relayer_components::chain::traits::send_message::CanSendMessages;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::packet::HasOutgoingPacketType;
use hermes_relayer_components::multi::types::tags::{Dst, Src};
use hermes_relayer_components::relay::impls::update_client::skip::LogSkipBuildUpdateClientMessage;
use hermes_relayer_components::relay::impls::update_client::wait::LogWaitUpdateClientHeightStatus;
use hermes_relayer_components::relay::traits::chains::{
    CanRaiseRelayChainErrors, HasRelayChains, HasRelayClientIds,
};
use hermes_relayer_components::relay::traits::ibc_message_sender::{CanSendIbcMessages, MainSink};
use hermes_relayer_components::relay::traits::packet_filter::PacketFilter;
use hermes_relayer_components::relay::traits::target::{
    DestinationTarget, HasDestinationTargetChainTypes, HasSourceTargetChainTypes, HasTargetChains,
    SourceTarget,
};
use hermes_relayer_components::relay::traits::update_client_message_builder::CanBuildTargetUpdateClientMessage;
use hermes_runtime_components::traits::channel::CanUseChannels;
use hermes_runtime_components::traits::channel_once::{CanCreateChannelsOnce, CanUseChannelsOnce};
use hermes_runtime_components::traits::runtime::HasRuntime;
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
    Relay: HasRuntime
        + HasMessageBatchSender<Src>
        + HasMessageBatchSender<Dst>
        + HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasSourceTargetChainTypes
        + HasDestinationTargetChainTypes
        + HasRelayClientIds
        + HasLogger<Logger = Logger>
        + HasTargetChains<SourceTarget>
        + HasTargetChains<DestinationTarget>
        + CanBuildTargetUpdateClientMessage<SourceTarget>
        + CanBuildTargetUpdateClientMessage<DestinationTarget>
        + CanRaiseError<SrcChain::Error>
        + CanRaiseError<DstChain::Error>
        + CanRaiseError<ErrorOf<Relay::Runtime>>
        + HasComponents<Components = Components>,
    SrcChain: HasIbcChainTypes<DstChain>
        + HasOutgoingPacketType<DstChain>
        + CanSendMessages
        + UseExtraChainComponentsForIbcMessageSender<DstChain>
        + CanRaiseError<ErrorOf<SrcChain::Runtime>>,
    DstChain: HasIbcChainTypes<SrcChain>
        + CanSendMessages
        + UseExtraChainComponentsForIbcMessageSender<SrcChain>
        + CanRaiseError<ErrorOf<DstChain::Runtime>>,
    SrcChain::Height: Clone,
    DstChain::Height: Clone,
    Relay::Runtime: CanSleep + CanCreateChannelsOnce + CanUseChannels + CanUseChannelsOnce,
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
