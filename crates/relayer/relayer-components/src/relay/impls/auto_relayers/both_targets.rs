use alloc::boxed::Box;
use alloc::vec;
use core::time::Duration;

use cgp::extra::run::{Runner, RunnerComponent};
use hermes_prelude::*;
use hermes_runtime_components::traits::{CanRunConcurrentTasks, HasRuntime, Task};

use crate::relay::traits::{
    CanAutoRelayTarget, CanRaiseRelayChainErrors, CanRefreshClient, DestinationTarget,
    HasRelayClientIds, SourceTarget,
};

pub struct RelayBothTargets;

pub enum EitherTarget {
    Source,
    Destination,
}

pub struct TargetRelayerTask<Relay> {
    pub relay: Relay,
    pub target: EitherTarget,
}

impl<Relay> Task for TargetRelayerTask<Relay>
where
    Relay: HasRelayClientIds
        + CanRaiseRelayChainErrors
        + HasRuntime
        + CanAutoRelayTarget<SourceTarget>
        + CanAutoRelayTarget<DestinationTarget>
        + CanRefreshClient<SourceTarget>
        + CanRefreshClient<DestinationTarget>,
{
    async fn run(self) {
        match self.target {
            EitherTarget::Source => {
                let auto_relay_task = self.relay.auto_relay(SourceTarget);
                let auto_refresh_task = self
                    .relay
                    .auto_refresh_client(SourceTarget, Duration::from_secs(10));

                let _ = futures::join!(auto_relay_task, auto_refresh_task);
            }
            EitherTarget::Destination => {
                let auto_relay_task = self.relay.auto_relay(DestinationTarget);
                let auto_refresh_task = self
                    .relay
                    .auto_refresh_client(SourceTarget, Duration::from_secs(10));

                let _ = futures::join!(auto_relay_task, auto_refresh_task);
            }
        }
    }
}

#[cgp_provider(RunnerComponent)]
impl<Relay> Runner<Relay> for RelayBothTargets
where
    Relay: Clone
        + HasRelayClientIds
        + HasRuntime
        + CanAutoRelayTarget<SourceTarget>
        + CanAutoRelayTarget<DestinationTarget>
        + CanRefreshClient<SourceTarget>
        + CanRefreshClient<DestinationTarget>
        + CanRaiseRelayChainErrors,
    Relay::Runtime: CanRunConcurrentTasks,
{
    async fn run(relay: &Relay) -> Result<(), Relay::Error> {
        let tasks = vec![
            Box::new(TargetRelayerTask {
                relay: relay.clone(),
                target: EitherTarget::Source,
            }),
            Box::new(TargetRelayerTask {
                relay: relay.clone(),
                target: EitherTarget::Destination,
            }),
        ];

        relay.runtime().run_concurrent_tasks(tasks).await;

        Ok(())
    }
}
