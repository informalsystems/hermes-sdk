use alloc::boxed::Box;
use core::marker::PhantomData;

use cgp::core::error::ErrorOf;
use cgp::extra::runtime::HasRuntime;
use cgp::prelude::*;
use hermes_chain_components::traits::queries::block_events::CanQueryBlockEvents;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::height::CanIncrementHeight;
use hermes_chain_components::types::aliases::EventOf;
use hermes_runtime_components::traits::task::{CanRunConcurrentTasks, Task};

use crate::components::default::relay::AutoRelayerComponent;
use crate::relay::traits::auto_relayer::AutoRelayer;
use crate::relay::traits::event_relayer::CanRelayEvent;
use crate::relay::traits::target::{HasTargetChainTypes, HasTargetChains, RelayTarget};

pub struct RelayWithPolledEvents;

#[cgp_provider(AutoRelayerComponent)]
impl<Relay, Target> AutoRelayer<Relay, Target> for RelayWithPolledEvents
where
    Relay: Clone
        + HasRuntime
        + HasTargetChains<Target>
        + CanRelayEvent<Target>
        + CanRaiseAsyncError<ErrorOf<Relay::TargetChain>>,
    Target: RelayTarget,
    Relay::TargetChain: CanQueryChainHeight + CanIncrementHeight + CanQueryBlockEvents,
    Relay::Runtime: CanRunConcurrentTasks,
{
    async fn auto_relay(relay: &Relay, _target: Target) -> Result<(), Relay::Error> {
        let chain = relay.target_chain();
        let runtime = relay.runtime();

        let mut height = chain
            .query_chain_height()
            .await
            .map_err(Relay::raise_error)?;

        loop {
            let events = chain
                .query_block_events(&height)
                .await
                .map_err(Relay::raise_error)?;

            let tasks = events
                .into_iter()
                .map(|event| {
                    Box::new(EventRelayerTask {
                        relay: relay.clone(),
                        event,
                        phantom: PhantomData,
                    })
                })
                .collect();

            runtime.run_concurrent_tasks(tasks).await;

            height = Relay::TargetChain::increment_height(&height).map_err(Relay::raise_error)?;
        }
    }
}

pub struct EventRelayerTask<Relay, Target>
where
    Target: RelayTarget,
    Relay: HasTargetChainTypes<Target, TargetChain: HasEventType>,
{
    pub relay: Relay,
    pub event: EventOf<Relay::TargetChain>,
    pub phantom: PhantomData<Target>,
}

impl<Relay, Target> Task for EventRelayerTask<Relay, Target>
where
    Target: RelayTarget,
    Relay: CanRelayEvent<Target>,
{
    async fn run(self) {
        let _ = self.relay.relay_chain_event(&self.event).await;
    }
}
