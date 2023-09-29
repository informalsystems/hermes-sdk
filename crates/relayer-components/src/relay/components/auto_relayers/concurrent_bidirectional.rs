use core::future::Future;
use core::pin::Pin;

use async_trait::async_trait;
use futures_util::stream::{self, StreamExt};

use crate::core::traits::run::Runner;
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::components::auto_relayer::CanAutoRelay;
use crate::relay::traits::target::{DestinationTarget, SourceTarget};
use crate::std_prelude::*;

/// A concurrent bidirectional relay context that supports auto-relaying between two
/// targets, a `SourceTarget` and a `DestinationTarget`. It is composed of two
/// unidirectional relay contexts.
///
/// Note that the `InRelayer` parameter is constrained such that the two sub-relay
/// contexts must relay between the same two connected chains, the `SourceTarget`
/// chain and the `DestinationTarget` chain, except in different directions.
///
/// This relayer type aggregates the relay tasks bound for the source chain and the tasks
/// bound for the target chain, collects them into a single stream, and then relays the
/// stream of tasks in cooperative multi-tasking fashion, which is how it achieves
/// concurrency.
pub struct ConcurrentBidirectionalRelayer;

#[async_trait]
impl<Relay> Runner<Relay> for ConcurrentBidirectionalRelayer
where
    Relay: HasRelayChains + CanAutoRelay<SourceTarget> + CanAutoRelay<DestinationTarget>,
{
    async fn run(relay: &Relay) -> Result<(), Relay::Error> {
        let src_task: Pin<Box<dyn Future<Output = ()> + Send>> = Box::pin(async move {
            let _ = relay.auto_relay(SourceTarget).await;
        });

        let dst_task: Pin<Box<dyn Future<Output = ()> + Send>> = Box::pin(async move {
            let _ = relay.auto_relay(DestinationTarget).await;
        });

        stream::iter([src_task, dst_task])
            .for_each_concurrent(None, |task| task)
            .await;

        Ok(())
    }
}
