use core::time::Duration;

use cgp_core::prelude::*;
use ibc_relayer_components::runtime::traits::sleep::Sleeper;
use tokio::time::sleep;

pub struct TokioSleep;

#[async_trait]
impl<Runtime> Sleeper<Runtime> for TokioSleep
where
    Runtime: Async,
{
    async fn sleep(_runtime: &Runtime, duration: Duration) {
        sleep(duration).await;
    }
}
