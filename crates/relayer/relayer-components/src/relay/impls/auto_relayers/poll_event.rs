use alloc::boxed::Box;
use alloc::format;
use core::marker::PhantomData;

use cgp::core::error::ErrorOf;
use cgp::prelude::*;
use hermes_chain_components::traits::queries::block_events::CanQueryBlockEvents;
use hermes_chain_components::traits::types::event::HasEventType;
use hermes_chain_components::traits::types::height::CanIncrementHeight;
use hermes_chain_components::types::aliases::{EventOf, HeightOf};
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::{LevelInfo, LevelTrace};
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::task::{CanRunConcurrentTasks, Task};

use crate::relay::traits::auto_relayer::{AutoRelayerWithHeights, AutoRelayerWithHeightsComponent};
use crate::relay::traits::event_relayer::CanRelayEvent;
use crate::relay::traits::target::{HasTargetChainTypes, HasTargetChains, RelayTarget};

pub struct RelayWithPolledEvents;

#[cgp_provider(AutoRelayerWithHeightsComponent)]
impl<Relay, Target> AutoRelayerWithHeights<Relay, Target> for RelayWithPolledEvents
where
    Relay: Clone
        + HasRuntime
        + HasTargetChains<Target>
        + HasLogger
        + CanRelayEvent<Target>
        + CanRaiseAsyncError<ErrorOf<Relay::TargetChain>>,
    Target: RelayTarget,
    Relay::TargetChain: CanIncrementHeight + CanQueryBlockEvents,
    Relay::Runtime: CanRunConcurrentTasks,
    Relay::Logger: CanLog<LevelInfo> + CanLog<LevelTrace>,
{
    async fn auto_relay_with_heights(
        relay: &Relay,
        _target: Target,
        start_height: &HeightOf<Relay::TargetChain>,
        end_height: Option<&HeightOf<Relay::TargetChain>>,
    ) -> Result<(), Relay::Error> {
        let chain = relay.target_chain();
        let runtime = relay.runtime();

        let mut height = start_height.clone();

        relay
            .logger()
            .log(
                &format!("Will start relaying at height `{height}`"),
                &LevelInfo,
            )
            .await;

        loop {
            let events = chain
                .query_block_events(&height)
                .await
                .map_err(Relay::raise_error)?;

            relay
                .logger()
                .log(
                    &format!("Queried {} events at height `{height}`", events.len()),
                    &LevelTrace,
                )
                .await;

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

            if let Some(end_height) = end_height {
                if &height > end_height {
                    relay
                        .logger()
                        .log(
                            &format!("Done clearing packets at height `{height}`"),
                            &LevelInfo,
                        )
                        .await;
                    return Ok(());
                }
            }
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
