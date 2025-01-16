use cgp::core::component::HasComponents;
use cgp::core::error::ErrorRaiser;
use cgp::extra::run::CanRun;
use cgp::prelude::HasAsyncErrorType;
use hermes_chain_type_components::traits::types::message::HasMessageType;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;
use hermes_relayer_components::chain::traits::event_subscription::HasEventSubscription;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::multi::types::tags::{Dst, Src};
use hermes_relayer_components::relay::traits::chains::{HasRelayChains, HasRelayClientIds};
use hermes_relayer_components::relay::traits::target::{
    DestinationTarget, HasDestinationTargetChainTypes, HasSourceTargetChainTypes, HasTargetChains,
    SourceTarget,
};
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::spawn::CanSpawnTask;
use hermes_runtime_components::traits::stream::CanMapStream;
use hermes_runtime_components::traits::subscription::HasSubscription;
use hermes_runtime_components::traits::task::CanRunConcurrentTasks;

use crate::batch::traits::types::CanUseMessageBatchChannel;
use crate::components::extra::closures::relay::event_relayer::UseExtraEventRelayer;
use crate::components::extra::relay::DelegatesToExtraRelayPreset;

pub trait CanUseExtraAutoRelayer: UseExtraAutoRelayer {}

pub trait UseExtraAutoRelayer: CanRun {}

impl<Relay, SrcChain, DstChain, Components> UseExtraAutoRelayer for Relay
where
    Relay: Clone
        + HasRuntime
        + HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasRelayClientIds
        + HasSourceTargetChainTypes
        + HasDestinationTargetChainTypes
        + HasTargetChains<SourceTarget>
        + HasTargetChains<DestinationTarget>
        + CanUseMessageBatchChannel<Src>
        + CanUseMessageBatchChannel<Dst>
        + UseExtraEventRelayer
        + HasComponents<Components = Components>,
    SrcChain: HasAsyncErrorType
        + HasRuntime
        + HasChainId
        + HasMessageType
        + HasMessageResponseType
        + HasEventSubscription,
    DstChain: HasAsyncErrorType
        + HasRuntime
        + HasChainId
        + HasMessageType
        + HasMessageResponseType
        + HasEventSubscription,
    Relay::Runtime: CanSpawnTask + CanRunConcurrentTasks,
    SrcChain::Runtime: HasSubscription + CanRunConcurrentTasks + CanMapStream,
    DstChain::Runtime: HasSubscription + CanRunConcurrentTasks + CanMapStream,
    Components: DelegatesToExtraRelayPreset
        + ErrorRaiser<Relay, SrcChain::Error>
        + ErrorRaiser<Relay, DstChain::Error>,
{
}
