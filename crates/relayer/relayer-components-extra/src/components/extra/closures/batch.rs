use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::message::CanEstimateMessageSize;
use hermes_relayer_components::logger::traits::has_logger::HasLogger;
use hermes_relayer_components::logger::traits::level::HasBaseLogLevels;
use hermes_relayer_components::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use hermes_relayer_components::relay::traits::components::ibc_message_sender::CanSendIbcMessages;
use hermes_relayer_components::relay::traits::target::{DestinationTarget, SourceTarget};
use hermes_relayer_components::runtime::traits::mutex::HasMutex;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_relayer_components::runtime::traits::sleep::CanSleep;
use hermes_relayer_components::runtime::traits::time::HasTime;
use hermes_relayer_components::runtime::types::aliases::Runtime;

use crate::batch::types::sink::BatchWorkerSink;
use crate::batch::worker::CanSpawnBatchMessageWorker;
use crate::runtime::traits::channel::{CanCloneSender, CanUseChannels, HasChannelTypes};
use crate::runtime::traits::channel_once::{CanUseChannelsOnce, HasChannelOnceTypes};
use crate::runtime::traits::spawn::CanSpawnTask;

pub trait CanUseBatchMessageWorkerSpawner: UseBatchMessageWorkerSpawner
where
    Self::SrcChain: HasRuntime,
    Self::DstChain: HasRuntime,
    Runtime<Self::SrcChain>: HasChannelTypes + HasChannelOnceTypes,
    Runtime<Self::DstChain>: HasChannelTypes + HasChannelOnceTypes,
{
}

pub trait UseBatchMessageWorkerSpawner:
    CanSpawnBatchMessageWorker<SourceTarget>
    + CanSpawnBatchMessageWorker<DestinationTarget>
    + CanRaiseRelayChainErrors
where
    Self::SrcChain: HasRuntime,
    Self::DstChain: HasRuntime,
    Runtime<Self::SrcChain>: HasChannelTypes + HasChannelOnceTypes,
    Runtime<Self::DstChain>: HasChannelTypes + HasChannelOnceTypes,
{
}

impl<Relay, SrcChain, DstChain> UseBatchMessageWorkerSpawner for Relay
where
    Relay: Clone
        + HasLogger
        + HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + CanSendIbcMessages<BatchWorkerSink, SourceTarget>
        + CanSendIbcMessages<BatchWorkerSink, DestinationTarget>
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
    Relay::Logger: HasBaseLogLevels,
{
}
