use alloc::vec;

use cgp_core::{async_trait, Runner};

use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use crate::relay::traits::components::auto_relayer::CanAutoRelay;
use crate::relay::traits::target::{DestinationTarget, SourceTarget};
use crate::runtime::traits::runtime::HasRuntime;
use crate::runtime::traits::task::{CanRunConcurrentTasks, Task};

pub struct RelayBothTargets;

pub enum EitherTarget {
    Source,
    Destination,
}

pub struct TargetRelayerTask<Relay> {
    pub relay: Relay,
    pub target: EitherTarget,
}

#[async_trait]
impl<Relay> Task for TargetRelayerTask<Relay>
where
    Relay: HasRelayChains
        + CanRaiseRelayChainErrors
        + HasRuntime
        + CanAutoRelay<SourceTarget>
        + CanAutoRelay<DestinationTarget>,
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

#[async_trait]
impl<Relay> Runner<Relay> for RelayBothTargets
where
    Relay: Clone
        + HasRelayChains
        + HasRuntime
        + CanAutoRelay<SourceTarget>
        + CanAutoRelay<DestinationTarget>
        + CanRaiseRelayChainErrors,
    Relay::Runtime: CanRunConcurrentTasks,
{
    async fn run(relay: &Relay) -> Result<(), Relay::Error> {
        let tasks = vec![
            TargetRelayerTask {
                relay: relay.clone(),
                target: EitherTarget::Source,
            },
            TargetRelayerTask {
                relay: relay.clone(),
                target: EitherTarget::Destination,
            },
        ];

        relay.runtime().run_concurrent_tasks(tasks).await;

        Ok(())
    }
}
