use cgp_core::{CanRun, HasComponents};
use hermes_relayer_components::chain::traits::event_subscription::HasEventSubscription;
use hermes_relayer_components::chain::traits::logs::event::CanLogChainEvent;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::logger::traits::has_logger::{HasLogger, HasLoggerType};
use hermes_relayer_components::logger::traits::level::HasBaseLogLevels;
use hermes_relayer_components::relay::traits::chains::HasRelayChains;
use hermes_relayer_components::runtime::traits::runtime::{HasRuntime, HasRuntimeType};
use hermes_relayer_components::runtime::traits::stream::CanMapStream;
use hermes_relayer_components::runtime::traits::subscription::HasSubscription;
use hermes_relayer_components::runtime::traits::task::CanRunConcurrentTasks;

use crate::components::extra::closures::relay::event_relayer::UseExtraEventRelayer;
use crate::components::extra::relay::DelegatesToExtraRelayComponents;
use crate::runtime::traits::spawn::CanSpawnTask;

pub trait CanUseExtraAutoRelayer: UseExtraAutoRelayer {}

pub trait UseExtraAutoRelayer: CanRun {}

impl<Relay, Components> UseExtraAutoRelayer for Relay
where
    Relay: Clone
        + HasRuntime
        + HasLogger
        + HasRelayChains
        + UseExtraEventRelayer
        + HasComponents<Components = Components>,
    Relay::SrcChain: HasRuntime
        + HasChainId
        + HasLoggerType<Logger = Relay::Logger>
        + CanLogChainEvent
        + HasEventSubscription,
    Relay::DstChain: HasRuntime
        + HasChainId
        + HasLoggerType<Logger = Relay::Logger>
        + CanLogChainEvent
        + HasEventSubscription,
    Relay::Runtime: CanSpawnTask + CanRunConcurrentTasks,
    Relay::Logger: HasBaseLogLevels,
    <Relay::SrcChain as HasRuntimeType>::Runtime:
        HasSubscription + CanRunConcurrentTasks + CanMapStream,
    <Relay::DstChain as HasRuntimeType>::Runtime:
        HasSubscription + CanRunConcurrentTasks + CanMapStream,
    Components: DelegatesToExtraRelayComponents,
{
}
