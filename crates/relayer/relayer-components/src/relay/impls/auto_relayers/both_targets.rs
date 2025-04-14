use alloc::boxed::Box;
use alloc::vec;

use cgp::extra::run::{Runner, RunnerComponent};
use cgp::prelude::*;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::task::{CanRunConcurrentTasks, Task};

use crate::relay::traits::{
    CanAutoRelayTarget, CanRaiseRelayChainErrors, DestinationTarget, HasRelayClientIds,
    SourceTarget,
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
        + CanAutoRelayTarget<DestinationTarget>,
{
    async fn run(self) {
        match self.target {
            EitherTarget::Source => {
                let _ = self.relay.auto_relay(SourceTarget).await;
            }
            EitherTarget::Destination => {
                let _ = self.relay.auto_relay(DestinationTarget).await;
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
