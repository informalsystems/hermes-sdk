use alloc::boxed::Box;
use alloc::vec;
use core::time::Duration;

use cgp::extra::run::{Runner, RunnerComponent};
use hermes_prelude::*;
use hermes_runtime_components::traits::{CanRunConcurrentTasks, HasRuntime, Task};

use crate::multi::traits::refresh_rate::{HasRefreshRateAToB, HasRefreshRateBtoA};
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
    pub refresh_rate_a: Option<Duration>,
    pub refresh_rate_b: Option<Duration>,
}

impl<Relay> Task for TargetRelayerTask<Relay>
where
    Relay: HasRelayClientIds
        + CanRaiseRelayChainErrors
        + HasRuntime
        + CanAutoRelayTarget<SourceTarget>
        + CanAutoRelayTarget<DestinationTarget>,
{
    async fn run(self) {
        match self.target {
            EitherTarget::Source => {
                let _ = self
                    .relay
                    .auto_relay(SourceTarget, self.refresh_rate_a)
                    .await;
            }
            EitherTarget::Destination => {
                let _ = self
                    .relay
                    .auto_relay(DestinationTarget, self.refresh_rate_b)
                    .await;
            }
        }
    }
}

#[cgp_provider(RunnerComponent)]
impl<Relay> Runner<Relay> for RelayBothTargets
where
    Relay: Clone
        + HasRelayClientIds
        + HasRefreshRateAToB
        + HasRefreshRateBtoA
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
                refresh_rate_a: *relay.refresh_rate_a(),
                refresh_rate_b: *relay.refresh_rate_b(),
            }),
            Box::new(TargetRelayerTask {
                relay: relay.clone(),
                target: EitherTarget::Destination,
                refresh_rate_a: *relay.refresh_rate_a(),
                refresh_rate_b: *relay.refresh_rate_b(),
            }),
        ];

        relay.runtime().run_concurrent_tasks(tasks).await;

        Ok(())
    }
}
