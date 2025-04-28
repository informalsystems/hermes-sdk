use core::time::Duration;

use cgp::prelude::*;
use hermes_runtime_components::traits::{Sleeper, SleeperComponent};
use tokio::time::sleep;

pub struct TokioSleep;

#[cgp_provider(SleeperComponent)]
impl<Runtime> Sleeper<Runtime> for TokioSleep
where
    Runtime: Async,
{
    async fn sleep(_runtime: &Runtime, duration: Duration) {
        sleep(duration).await;
    }
}
