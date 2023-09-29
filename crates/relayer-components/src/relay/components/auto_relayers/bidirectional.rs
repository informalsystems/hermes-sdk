use async_trait::async_trait;
use futures_util::stream;

use crate::core::traits::run::Runner;
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::components::auto_relayer::CanAutoRelay;
use crate::relay::traits::target::{DestinationTarget, SourceTarget};
use crate::runtime::traits::runtime::HasRuntime;
use crate::runtime::traits::task::{CanRunConcurrentTasks, Task};
use crate::std_prelude::*;

pub struct BidirectionalRelayer;

pub enum EitherTarget {
    Source,
    Destination,
}

pub struct RunAutoRelayerWithTarget<Relay> {
    pub relay: Relay,
    pub target: EitherTarget,
}

#[async_trait]
impl<Relay> Task for RunAutoRelayerWithTarget<Relay>
where
    Relay:
        HasRelayChains + HasRuntime + CanAutoRelay<SourceTarget> + CanAutoRelay<DestinationTarget>,
{
    async fn run(&self) {
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
impl<Relay> Runner<Relay> for BidirectionalRelayer
where
    Relay: Clone
        + HasRelayChains
        + HasRuntime
        + CanAutoRelay<SourceTarget>
        + CanAutoRelay<SourceTarget>,
    Relay::Runtime: CanRunConcurrentTasks<RunAutoRelayerWithTarget<Relay>>,
{
    async fn run(relay: &Relay) -> Result<(), Relay::Error> {
        let tasks = vec![
            RunAutoRelayerWithTarget {
                relay: relay.clone(),
                target: EitherTarget::Source,
            },
            RunAutoRelayerWithTarget {
                relay: relay.clone(),
                target: EitherTarget::Destination,
            },
        ];

        relay
            .runtime()
            .run_concurrent_tasks(stream::iter(tasks))
            .await;

        Ok(())
    }
}
