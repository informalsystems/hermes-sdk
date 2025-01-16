use alloc::boxed::Box;
use core::marker::PhantomData;

use cgp::prelude::HasAsyncErrorType;
use hermes_chain_components::types::aliases::{EventOf, HeightOf};
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::stream::CanMapStream;
use hermes_runtime_components::traits::subscription::HasSubscription;
use hermes_runtime_components::traits::task::{CanRunConcurrentTasks, Task};

use crate::chain::traits::event_subscription::HasEventSubscription;
use crate::chain::traits::types::event::HasEventType;
use crate::chain::traits::types::height::HasHeightType;
use crate::relay::traits::auto_relayer::AutoRelayer;
use crate::relay::traits::event_relayer::CanRelayEvent;
use crate::relay::traits::target::{HasTargetChainTypes, HasTargetChains, RelayTarget};

/// A one-way auto-relayer type that is responsible for listening for a
/// particular event subscription and relaying messages to a target
/// chain in response to those events in a concurrent fashion.
pub struct RelayEvents;

pub struct EventRelayerTask<Relay, Target>
where
    Target: RelayTarget,
    Relay: HasTargetChainTypes<Target, TargetChain: HasHeightType + HasEventType>,
{
    pub relay: Relay,
    pub height: HeightOf<Relay::TargetChain>,
    pub event: EventOf<Relay::TargetChain>,
    pub phantom: PhantomData<Target>,
}

impl<Relay, Target> Task for EventRelayerTask<Relay, Target>
where
    Target: RelayTarget,
    Relay: CanRelayEvent<Target>,
{
    async fn run(self) {
        let _ = self
            .relay
            .relay_chain_event(&self.height, &self.event)
            .await;
    }
}

impl<Relay, Target, Runtime> AutoRelayer<Relay, Target> for RelayEvents
where
    Target: RelayTarget,
    Relay: HasTargetChains<Target> + CanRelayEvent<Target> + HasRuntime + Clone,
    Relay::TargetChain: HasEventSubscription<Runtime = Runtime>,
    Runtime: HasSubscription + CanMapStream + CanRunConcurrentTasks + HasAsyncErrorType,
{
    async fn auto_relay(relay: &Relay, _target: Target) -> Result<(), Relay::Error> {
        let target_chain = relay.target_chain();
        let subscription = target_chain.event_subscription();

        loop {
            if let Some(event_stream) = Runtime::subscribe(subscription).await {
                let tasks = {
                    let relay = relay.clone();

                    Runtime::map_stream(event_stream, move |(height, event)| {
                        Box::new(EventRelayerTask {
                            relay: relay.clone(),
                            height,
                            event,
                            phantom: PhantomData,
                        })
                    })
                };

                target_chain
                    .runtime()
                    .run_concurrent_task_stream(tasks)
                    .await;
            } else {
                return Ok(());
            }
        }
    }
}
