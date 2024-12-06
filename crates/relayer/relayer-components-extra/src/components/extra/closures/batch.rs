use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::message::CanEstimateMessageSize;
use hermes_relayer_components::multi::types::tags::{Dst, Src};
use hermes_relayer_components::relay::traits::chains::{
    CanRaiseRelayChainErrors, HasRelayChains, HasRelayClientIds,
};
use hermes_relayer_components::relay::traits::ibc_message_sender::CanSendIbcMessages;
use hermes_relayer_components::relay::traits::target::{
    DestinationTarget, HasDestinationTargetChainTypes, HasSourceTargetChainTypes, HasTargetChains,
    SourceTarget,
};
use hermes_runtime_components::traits::channel::{CanCloneSender, CanUseChannels, HasChannelTypes};
use hermes_runtime_components::traits::channel_once::{CanUseChannelsOnce, HasChannelOnceTypes};
use hermes_runtime_components::traits::mutex::HasMutex;
use hermes_runtime_components::traits::runtime::{HasRuntime, RuntimeOf};
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_runtime_components::traits::spawn::CanSpawnTask;
use hermes_runtime_components::traits::time::HasTime;

use crate::batch::traits::types::CanUseMessageBatchChannel;
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
    HasRelayClientIds
    + CanSpawnBatchMessageWorker<SourceTarget>
    + CanSpawnBatchMessageWorker<DestinationTarget>
    + CanRaiseRelayChainErrors
{
}

impl<Relay> UseBatchMessageWorkerSpawner for Relay
where
    Relay: Clone
        + HasRuntime
        + CanUseMessageBatchChannel<Src>
        + CanUseMessageBatchChannel<Dst>
        + HasRelayChains
        + HasRelayClientIds
        + HasSourceTargetChainTypes
        + HasDestinationTargetChainTypes
        + HasTargetChains<SourceTarget>
        + HasTargetChains<DestinationTarget>
        + CanSendIbcMessages<BatchWorkerSink, SourceTarget>
        + CanSendIbcMessages<BatchWorkerSink, DestinationTarget>
        + HasLogger
        + CanRaiseRelayChainErrors,
    Relay::SrcChain: HasRuntime
        + HasChainId
        + HasMessageResponseType
        + CanEstimateMessageSize
        + HasIbcChainTypes<Relay::DstChain>,
    Relay::DstChain: HasRuntime
        + HasChainId
        + HasMessageResponseType
        + CanEstimateMessageSize
        + HasIbcChainTypes<Relay::SrcChain>,
    Relay::Runtime: HasTime
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
