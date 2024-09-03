use core::time::Duration;

use cgp::prelude::*;

#[derive_component(SleeperComponent, Sleeper<Runtime>)]
#[async_trait]
pub trait CanSleep: Async {
    async fn sleep(&self, duration: Duration);
}
