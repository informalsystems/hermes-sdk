use alloc::boxed::Box;
use core::time::Duration;

use cgp_core::prelude::*;

#[derive_component(SleeperComponent, Sleeper<Runtime>)]
#[async_trait]
pub trait CanSleep: Async {
    async fn sleep(&self, duration: Duration);
}
