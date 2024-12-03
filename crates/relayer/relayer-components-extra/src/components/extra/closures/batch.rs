use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::message::CanEstimateMessageSize;
use hermes_relayer_components::relay::traits::chains::{
    CanRaiseRelayChainErrors, HasRelayChains, HasRelayClientIds,
};
use hermes_relayer_components::relay::traits::ibc_message_sender::CanSendIbcMessages;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_runtime_components::traits::channel::{CanCloneSender, CanUseChannels, HasChannelTypes};
use hermes_runtime_components::traits::channel_once::{CanUseChannelsOnce, HasChannelOnceTypes};
use hermes_runtime_components::traits::mutex::HasMutex;
use hermes_runtime_components::traits::runtime::{HasRuntime, RuntimeOf};
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_runtime_components::traits::spawn::CanSpawnTask;
use hermes_runtime_components::traits::time::HasTime;

use crate::batch::types::sink::BatchWorkerSink;
use crate::batch::worker::{CanSpawnBatchMessageWorker, LogBatchWorker};

pub trait CanUseBatchMessageWorkerSpawner: UseBatchMessageWorkerSpawner
where
    Self::SrcChain: HasRuntime,
    Self::DstChain: HasRuntime,
    RuntimeOf<Self::SrcChain>: HasChannelTypes + HasChannelOnceTypes,
    RuntimeOf<Self::DstChain>: HasChannelTypes + HasChannelOnceTypes,
{
}

pub trait UseBatchMessageWorkerSpawner:
    CanSpawnBatchMessageWorker<SourceTarget>
    + CanSpawnBatchMessageWorker<DestinationTarget>
    + HasRelayClientIds
    + CanRaiseRelayChainErrors
where
    Self::SrcChain: HasRuntime,
    Self::DstChain: HasRuntime,
    RuntimeOf<Self::SrcChain>: HasChannelTypes + HasChannelOnceTypes,
    RuntimeOf<Self::DstChain>: HasChannelTypes + HasChannelOnceTypes,
{
}

impl<Relay, SrcChain, DstChain> UseBatchMessageWorkerSpawner for Relay
where
    Relay: Clone
        + HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasRelayClientIds
        + CanSendIbcMessages<BatchWorkerSink, SourceTarget>
        + CanSendIbcMessages<BatchWorkerSink, DestinationTarget>
        + HasLogger
        + CanRaiseRelayChainErrors,
    SrcChain: HasRuntime + HasChainId + CanEstimateMessageSize + HasIbcChainTypes<DstChain>,
    DstChain: HasRuntime + HasChainId + CanEstimateMessageSize + HasIbcChainTypes<SrcChain>,
    SrcChain::Runtime: HasTime
        + HasMutex
        + CanSleep
        + CanSpawnTask
        + CanUseChannels
        + CanUseChannelsOnce
        + CanCloneSender,
    DstChain::Runtime: HasTime
        + HasMutex
        + CanSleep
        + CanSpawnTask
        + CanUseChannels
        + CanUseChannelsOnce
        + CanCloneSender,
    Relay::Error: Clone,
    Relay::Logger: for<'a> CanLog<LogBatchWorker<'a, Relay, SourceTarget>>
        + for<'a> CanLog<LogBatchWorker<'a, Relay, DestinationTarget>>,
{
}
