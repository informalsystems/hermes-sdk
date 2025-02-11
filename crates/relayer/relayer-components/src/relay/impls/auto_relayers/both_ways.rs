use alloc::boxed::Box;
use alloc::vec;

use cgp::core::field::Index;
use cgp::extra::run::{CanRun, Runner, RunnerComponent};
use cgp::prelude::*;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::task::{CanRunConcurrentTasks, Task};

use crate::birelay::traits::two_way::HasTwoWayRelay;
use crate::multi::traits::relay_at::RelayAt;

/// A concurrent two-way relay context that is composed of a `BiRelay` type that
/// can auto-relay between two connected targets.
///
/// As opposed to the `ParallelTwoWayAutoRelay` variant, this concurrent variant
/// runs in a single thread and achieves concurrency via cooperative multi-tasking.
pub struct RelayBothWays;

pub enum TwoWayRelayerTask<BiRelay>
where
    BiRelay: HasTwoWayRelay,
{
    AToB(RelayAt<BiRelay, Index<0>, Index<1>>),
    BToA(RelayAt<BiRelay, Index<1>, Index<0>>),
}

impl<BiRelay> Task for TwoWayRelayerTask<BiRelay>
where
    BiRelay: HasTwoWayRelay,
    RelayAt<BiRelay, Index<0>, Index<1>>: CanRun,
    RelayAt<BiRelay, Index<1>, Index<0>>: CanRun,
{
    async fn run(self) {
        match self {
            Self::AToB(relay) => {
                let _ = relay.run().await;
            }
            Self::BToA(relay) => {
                let _ = relay.run().await;
            }
        }
    }
}

#[cgp_provider(RunnerComponent)]
impl<BiRelay> Runner<BiRelay> for RelayBothWays
where
    BiRelay: HasTwoWayRelay + HasRuntime + HasAsyncErrorType,
    RelayAt<BiRelay, Index<0>, Index<1>>: Clone + CanRun,
    RelayAt<BiRelay, Index<1>, Index<0>>: Clone + CanRun,
    BiRelay::Runtime: CanRunConcurrentTasks,
{
    async fn run(birelay: &BiRelay) -> Result<(), BiRelay::Error> {
        let tasks = vec![
            Box::new(<TwoWayRelayerTask<BiRelay>>::AToB(
                birelay.relay_a_to_b().clone(),
            )),
            Box::new(<TwoWayRelayerTask<BiRelay>>::BToA(
                birelay.relay_b_to_a().clone(),
            )),
        ];

        birelay.runtime().run_concurrent_tasks(tasks).await;

        Ok(())
    }
}
