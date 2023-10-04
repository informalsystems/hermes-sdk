use cgp_core::{Async, HasComponents};
use ibc_relayer_components::chain::traits::event_subscription::HasEventSubscription;
use ibc_relayer_components::chain::traits::logs::event::CanLogChainEvent;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainId;
use ibc_relayer_components::core::traits::run::CanRun;
use ibc_relayer_components::logger::traits::has_logger::{HasLogger, HasLoggerType};
use ibc_relayer_components::logger::traits::level::HasBaseLogLevels;
use ibc_relayer_components::relay::traits::chains::HasRelayChains;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_relayer_components::runtime::traits::stream::CanMapStream;
use ibc_relayer_components::runtime::traits::subscription::HasSubscriptionType;
use ibc_relayer_components::runtime::traits::task::CanRunConcurrentTasks;

use crate::components::extra::closures::relay::event_relayer::UseExtraEventRelayer;
use crate::components::extra::relay::ExtraRelayComponents;
use crate::runtime::traits::spawn::CanSpawnTask;

pub trait CanUseExtraAutoRelayer: UseExtraAutoRelayer {}

pub trait UseExtraAutoRelayer: CanRun {}

impl<Relay, BaseRelayComponents> UseExtraAutoRelayer for Relay
where
    Relay: Clone
        + HasRuntime
        + HasLogger
        + HasRelayChains
        + UseExtraEventRelayer
        + HasComponents<Components = ExtraRelayComponents<BaseRelayComponents>>,
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
    <Relay::SrcChain as HasRuntime>::Runtime:
        HasSubscriptionType + CanRunConcurrentTasks + CanMapStream,
    <Relay::DstChain as HasRuntime>::Runtime:
        HasSubscriptionType + CanRunConcurrentTasks + CanMapStream,
    BaseRelayComponents: Async,
{
}
