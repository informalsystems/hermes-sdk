use cgp_core::{CanRun, ErrorRaiser, HasComponents, HasErrorType};

use crate::chain::traits::event_subscription::HasEventSubscription;
use crate::components::default::closures::relay::event_relayer::UseDefaultEventRelayer;
use crate::components::default::relay::DelegatesToDefaultRelayComponents;
use crate::relay::traits::chains::HasRelayChains;
use crate::runtime::traits::runtime::HasRuntime;
use crate::runtime::traits::stream::CanMapStream;
use crate::runtime::traits::subscription::HasSubscription;
use crate::runtime::traits::task::CanRunConcurrentTasks;

pub trait CanUseDefaultAutoRelayer: UseDefaultAutoRelayer {}

pub trait UseDefaultAutoRelayer: CanRun {}

impl<Relay, SrcChain, DstChain, Components> UseDefaultAutoRelayer for Relay
where
    Relay: Clone
        + HasRuntime
        + HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
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
