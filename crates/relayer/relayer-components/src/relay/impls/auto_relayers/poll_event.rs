use alloc::boxed::Box;
use alloc::vec::Vec;
use alloc::{format, vec};
use core::marker::PhantomData;
use core::time::Duration;

use cgp::core::error::ErrorOf;
use hermes_chain_components::traits::{
    CanIncrementHeight, CanQueryBlockEvents, HasEventType, HasHeightType,
};
use hermes_chain_components::types::aliases::{EventOf, HeightOf};
use hermes_logging_components::traits::CanLog;
use hermes_logging_components::types::{LevelInfo, LevelTrace};
use hermes_prelude::*;
use hermes_runtime_components::traits::{
    CanRunConcurrentTasks, CanSleep, HasRuntime, HasTime, Task,
};

use crate::relay::traits::{
    AutoRelayerWithHeights, AutoRelayerWithHeightsComponent, CanRelayBatchEvent,
    HasTargetChainTypes, HasTargetChains, RelayTarget,
};

#[cgp_new_provider(AutoRelayerWithHeightsComponent)]
impl<Relay, Target> AutoRelayerWithHeights<Relay, Target> for RelayWithPolledEvents
where
    Relay: Clone
        + HasRuntime
        + HasTargetChains<Target>
        + CanRelayBatchEvent<Target>
        + CanLog<LevelInfo>
        + CanLog<LevelTrace>
        + for<'a> CanLog<LogAutoRelayWithHeights<'a, Relay, Target>>
        + CanRaiseAsyncError<ErrorOf<Relay::TargetChain>>,
    Target: RelayTarget,
    Relay::TargetChain: CanIncrementHeight + CanQueryBlockEvents,
    Relay::Runtime: CanRunConcurrentTasks + CanSleep + HasTime,
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

        let mut last_sent_time = runtime.now();
        let mut batch_events = vec![];
        loop {
            let maybe_events = chain
                .query_block_events(&height)
                .await
                .map_err(Relay::raise_error);

            // TODO: Introduce retry mechanism
            let mut events = match maybe_events {
                Ok(events) => {
                    relay
                        .log(
                            &format!("Queried {} events at height `{height}`", events.len()),
                            &LevelTrace,
                        )
                        .await;
                    height = Relay::TargetChain::increment_height(&height)
                        .map_err(Relay::raise_error)?;
                    events
                }
                Err(e) => {
                    relay
                        .log(
                            &format!(
                                "failed to retrieve events due to `{e:?}`. Will retry in 1 second"
                            ),
                            &LevelTrace,
                        )
                        .await;
                    runtime.sleep(Duration::from_secs(1)).await;
                    vec![]
                }
            };

            if !events.is_empty() {
                batch_events.append(&mut events);
            }
            let now = runtime.now();
            let elapsed = Relay::Runtime::duration_since(&now, &last_sent_time);
            if elapsed < Duration::from_secs(15) {
                continue;
            }
            last_sent_time = now;

            let tasks = Box::new(EventRelayerTask {
                relay: relay.clone(),
                events: batch_events.clone(),
                phantom: PhantomData,
            });

            runtime.run_concurrent_tasks(vec![tasks]).await;

            batch_events.clear();

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
    pub events: Vec<EventOf<Relay::TargetChain>>,
    pub phantom: PhantomData<Target>,
}

impl<Relay, Target> Task for EventRelayerTask<Relay, Target>
where
    Target: RelayTarget,
    Relay: CanRelayBatchEvent<Target>,
{
    async fn run(self) {
        let _ = self
            .relay
            .relay_chain_batch_events(self.events.iter().collect())
            .await;
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
