use core::marker::PhantomData;

use cgp_core::{async_trait, HasErrorType};

use crate::chain::traits::event_subscription::HasEventSubscription;
use crate::chain::traits::types::event::HasEventType;
use crate::chain::traits::types::height::HasHeightType;
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::components::auto_relayer::AutoRelayer;
use crate::relay::traits::components::event_relayer::CanRelayEvent;
use crate::relay::traits::target::ChainTarget;
use crate::runtime::traits::runtime::HasRuntime;
use crate::runtime::traits::stream::CanMapStream;
use crate::runtime::traits::subscription::HasSubscription;
use crate::runtime::traits::task::{CanRunConcurrentTasks, Task};
use crate::std_prelude::*;

/// A one-way auto-relayer type that is responsible for listening for a
/// particular event subscription and relaying messages to a target
/// chain in response to those events in a concurrent fashion.
pub struct RelayEvents;

pub struct EventRelayerTask<Relay, Target>
where
    Relay: HasRelayChains,
    Target: ChainTarget<Relay>,
    Target::TargetChain: HasHeightType + HasEventType,
{
    pub relay: Relay,
    pub height: <Target::TargetChain as HasHeightType>::Height,
    pub event: <Target::TargetChain as HasEventType>::Event,
    pub phantom: PhantomData<Target>,
}

#[async_trait]
impl<Relay, Target> Task for EventRelayerTask<Relay, Target>
where
    Relay: CanRelayEvent<Target>,
    Target: ChainTarget<Relay>,
{
    async fn run(self) {
        let _ = self
            .relay
            .relay_chain_event(&self.height, &self.event)
            .await;
    }
}

#[async_trait]
impl<Relay, Target, Runtime> AutoRelayer<Relay, Target> for RelayEvents
where
    Relay: CanRelayEvent<Target> + HasRuntime + Clone,
    Target: ChainTarget<Relay>,
    Target::TargetChain: HasEventSubscription<Runtime = Runtime>,
    Runtime: HasSubscription + CanMapStream + CanRunConcurrentTasks + HasErrorType,
{
    async fn auto_relay(relay: &Relay, _target: Target) -> Result<(), Relay::Error> {
        let subscription = Target::target_chain(relay).event_subscription();

        loop {
            if let Some(event_stream) = Runtime::subscribe(subscription).await {
                let tasks = {
                    let relay = relay.clone();

                    Runtime::map_stream(event_stream, move |(height, event)| EventRelayerTask {
                        relay: relay.clone(),
                        height,
                        event,
                        phantom: PhantomData,
                    })
                };

                Target::target_chain(relay)
                    .runtime()
                    .run_concurrent_task_stream(tasks)
                    .await;
            } else {
                return Ok(());
            }
        }
    }
}
