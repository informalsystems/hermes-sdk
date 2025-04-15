use alloc::boxed::Box;
use alloc::format;
use core::marker::PhantomData;

use cgp::core::error::ErrorOf;
use cgp::prelude::*;
use hermes_chain_components::traits::{
    CanIncrementHeight, CanQueryBlockEvents, HasEventType, HasHeightType,
};
use hermes_chain_components::types::aliases::{EventOf, HeightOf};
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::{LevelInfo, LevelTrace};
use hermes_runtime_components::traits::{CanRunConcurrentTasks, HasRuntime, Task};

use crate::relay::traits::{
    AutoRelayerWithHeights, AutoRelayerWithHeightsComponent, CanRelayEvent, HasTargetChainTypes,
    HasTargetChains, RelayTarget,
};

#[cgp_new_provider(AutoRelayerWithHeightsComponent)]
impl<Relay, Target> AutoRelayerWithHeights<Relay, Target> for RelayWithPolledEvents
where
    Relay: Clone
        + HasRuntime
        + HasTargetChains<Target>
        + CanRelayEvent<Target>
        + CanLog<LevelInfo>
        + CanLog<LevelTrace>
        + for<'a> CanLog<LogAutoRelayWithHeights<'a, Relay, Target>>
        + CanRaiseAsyncError<ErrorOf<Relay::TargetChain>>,
    Target: RelayTarget,
    Relay::TargetChain: CanIncrementHeight + CanQueryBlockEvents,
    Relay::Runtime: CanRunConcurrentTasks,
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
            .log(
                "starting auto relay with heights",
                &LogAutoRelayWithHeights {
                    relay,
                    start_height,
                    end_height,
                    phantom: PhantomData,
                },
            )
            .await;

        loop {
            let events = chain
                .query_block_events(&height)
                .await
                .map_err(Relay::raise_error)?;

            relay
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

pub struct LogAutoRelayWithHeights<'a, Relay, Target>
where
    Relay: HasTargetChainTypes<Target>,
    Target: RelayTarget,
    Relay::TargetChain: HasHeightType,
{
    pub relay: &'a Relay,
    pub start_height: &'a HeightOf<Relay::TargetChain>,
    pub end_height: Option<&'a HeightOf<Relay::TargetChain>>,
    pub phantom: PhantomData<Target>,
}
