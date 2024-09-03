use core::time::Duration;

use cgp::prelude::*;
use hermes_runtime_components::traits::sleep::Sleeper;
use tokio::time::sleep;

pub struct TokioSleep;

impl<Runtime> Sleeper<Runtime> for TokioSleep
where
    Runtime: Async,
{
    async fn sleep(_runtime: &Runtime, duration: Duration) {
        sleep(duration).await;
    }
}
