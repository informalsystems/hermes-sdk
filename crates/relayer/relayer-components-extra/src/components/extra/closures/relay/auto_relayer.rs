use cgp_core::prelude::HasErrorType;
use cgp_core::{CanRun, ErrorRaiser, HasComponents};
use hermes_relayer_components::chain::traits::event_subscription::HasEventSubscription;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::relay::traits::chains::HasRelayChains;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::spawn::CanSpawnTask;
use hermes_runtime_components::traits::stream::CanMapStream;
use hermes_runtime_components::traits::subscription::HasSubscription;
use hermes_runtime_components::traits::task::CanRunConcurrentTasks;

use crate::components::extra::closures::relay::event_relayer::UseExtraEventRelayer;
use crate::components::extra::relay::DelegatesToExtraRelayComponents;

pub trait CanUseExtraAutoRelayer: UseExtraAutoRelayer {}

pub trait UseExtraAutoRelayer: CanRun {}

impl<Relay, SrcChain, DstChain, Components> UseExtraAutoRelayer for Relay
where
    Relay: Clone
        + HasRuntime
        + HasRelayChains<SrcChain = SrcChain, DstChain = DstChain>
        + UseExtraEventRelayer
        + HasComponents<Components = Components>,
    SrcChain: HasErrorType + HasRuntime + HasChainId + HasEventSubscription,
    DstChain: HasErrorType + HasRuntime + HasChainId + HasEventSubscription,
    Relay::Runtime: CanSpawnTask + CanRunConcurrentTasks,
    SrcChain::Runtime: HasSubscription + CanRunConcurrentTasks + CanMapStream,
    DstChain::Runtime: HasSubscription + CanRunConcurrentTasks + CanMapStream,
    Components: DelegatesToExtraRelayComponents
        + ErrorRaiser<Relay, SrcChain::Error>
        + ErrorRaiser<Relay, DstChain::Error>,
{
}
