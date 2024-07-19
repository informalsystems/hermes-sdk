use alloc::vec;

use cgp_core::error::HasErrorType;
use cgp_core::run::{CanRun, Runner};
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::task::{CanRunConcurrentTasks, Task};

use crate::birelay::traits::two_way::HasTwoWayRelay;
use crate::multi::traits::relay_at::RelayTypeAt;

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
    AToB(RelayTypeAt<BiRelay, 0, 1>),
    BToA(RelayTypeAt<BiRelay, 1, 0>),
}

impl<BiRelay> Task for TwoWayRelayerTask<BiRelay>
where
    BiRelay: HasTwoWayRelay,
    RelayTypeAt<BiRelay, 0, 1>: CanRun,
    RelayTypeAt<BiRelay, 1, 0>: CanRun,
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

impl<BiRelay> Runner<BiRelay> for RelayBothWays
where
    BiRelay: HasTwoWayRelay + HasRuntime + HasErrorType,
    RelayTypeAt<BiRelay, 0, 1>: Clone + CanRun,
    RelayTypeAt<BiRelay, 1, 0>: Clone + CanRun,
    BiRelay::Runtime: CanRunConcurrentTasks,
{
    async fn run(birelay: &BiRelay) -> Result<(), BiRelay::Error> {
        let tasks = vec![
            <TwoWayRelayerTask<BiRelay>>::AToB(birelay.relay_a_to_b().clone()),
            <TwoWayRelayerTask<BiRelay>>::BToA(birelay.relay_b_to_a().clone()),
        ];

        birelay.runtime().run_concurrent_tasks(tasks).await;

        Ok(())
    }
}
