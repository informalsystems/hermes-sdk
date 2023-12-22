use cgp_core::{CanRun, HasComponents};

use crate::chain::traits::event_subscription::HasEventSubscription;
use crate::components::default::closures::relay::event_relayer::UseDefaultEventRelayer;
use crate::components::default::relay::DelegatesToDefaultRelayComponents;
use crate::relay::traits::chains::HasRelayChains;
use crate::runtime::traits::runtime::{HasRuntime, HasRuntimeType};
use crate::runtime::traits::stream::CanMapStream;
use crate::runtime::traits::subscription::HasSubscription;
use crate::runtime::traits::task::CanRunConcurrentTasks;

pub trait CanUseDefaultAutoRelayer: UseDefaultAutoRelayer {}

pub trait UseDefaultAutoRelayer: CanRun {}

impl<Relay, Components> UseDefaultAutoRelayer for Relay
where
    Relay: Clone
        + HasRuntime
        + HasRelayChains
        + UseDefaultEventRelayer
        + HasComponents<Components = Components>,
    Relay::SrcChain: HasEventSubscription,
    Relay::DstChain: HasEventSubscription,
    Relay::Runtime: CanRunConcurrentTasks,
    <Relay::SrcChain as HasRuntimeType>::Runtime:
        HasSubscription + CanRunConcurrentTasks + CanMapStream,
    <Relay::DstChain as HasRuntimeType>::Runtime:
        HasSubscription + CanRunConcurrentTasks + CanMapStream,
    Components: DelegatesToDefaultRelayComponents,
{
}
