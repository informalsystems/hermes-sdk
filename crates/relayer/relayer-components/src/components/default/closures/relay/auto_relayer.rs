use cgp::core::component::HasComponents;
use cgp::core::error::{ErrorRaiser, HasErrorType};
use cgp::extra::run::CanRun;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::stream::CanMapStream;
use hermes_runtime_components::traits::subscription::HasSubscription;
use hermes_runtime_components::traits::task::CanRunConcurrentTasks;

use crate::chain::traits::event_subscription::HasEventSubscription;
use crate::components::default::closures::relay::event_relayer::UseDefaultEventRelayer;
use crate::components::default::relay::DelegatesToDefaultRelayComponents;
use crate::relay::traits::chains::{HasRelayChains, HasRelayClientIds};
use crate::relay::traits::target::{
    DestinationTarget, HasDestinationTargetChainTypes, HasSourceTargetChainTypes, HasTargetChains,
    SourceTarget,
};

pub trait CanUseDefaultAutoRelayer: UseDefaultAutoRelayer {}

pub trait UseDefaultAutoRelayer: CanRun {}

impl<Relay, SrcChain, DstChain, Components> UseDefaultAutoRelayer for Relay
where
    Relay: Clone
        + HasRuntime
        + HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + HasRelayClientIds
        + HasSourceTargetChainTypes
        + HasDestinationTargetChainTypes
        + HasTargetChains<SourceTarget>
        + HasTargetChains<DestinationTarget>
        + UseDefaultEventRelayer
        + HasComponents<Components = Components>,
    SrcChain: HasEventSubscription + HasErrorType,
    DstChain: HasEventSubscription + HasErrorType,
    Relay::Runtime: CanRunConcurrentTasks,
    SrcChain::Runtime: HasSubscription + CanRunConcurrentTasks + CanMapStream,
    DstChain::Runtime: HasSubscription + CanRunConcurrentTasks + CanMapStream,
    Components: DelegatesToDefaultRelayComponents
        + ErrorRaiser<Relay, SrcChain::Error>
        + ErrorRaiser<Relay, DstChain::Error>,
{
}
