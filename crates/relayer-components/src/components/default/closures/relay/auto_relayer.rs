use cgp_core::traits::has_components::HasComponents;
use cgp_core::traits::Async;

use crate::chain::traits::event_subscription::HasEventSubscription;
use crate::components::default::closures::relay::event_relayer::UseDefaultEventRelayer;
use crate::components::default::relay::DefaultRelayComponents;
use crate::core::traits::run::CanRun;
use crate::relay::traits::chains::HasRelayChains;
use crate::runtime::traits::runtime::HasRuntime;
use crate::runtime::traits::stream::CanMapStream;
use crate::runtime::traits::subscription::HasSubscriptionType;
use crate::runtime::traits::task::CanRunConcurrentTasks;

pub trait CanUseDefaultAutoRelayer: UseDefaultAutoRelayer {}

pub trait UseDefaultAutoRelayer: CanRun {}

impl<Relay, BaseRelayComponents> UseDefaultAutoRelayer for Relay
where
    Relay: Clone
        + HasRuntime
        + HasRelayChains
        + UseDefaultEventRelayer
        + HasComponents<Components = DefaultRelayComponents<BaseRelayComponents>>,
    Relay::SrcChain: HasEventSubscription,
    Relay::DstChain: HasEventSubscription,
    Relay::Runtime: CanRunConcurrentTasks,
    <Relay::SrcChain as HasRuntime>::Runtime:
        HasSubscriptionType + CanRunConcurrentTasks + CanMapStream,
    <Relay::DstChain as HasRuntime>::Runtime:
        HasSubscriptionType + CanRunConcurrentTasks + CanMapStream,
    BaseRelayComponents: Async,
{
}
