use core::time::Duration;

use cgp::prelude::*;

#[cgp_component {
  provider: Sleeper,
  context: Runtime,
}]
#[async_trait]
pub trait CanSleep: Async {
    async fn sleep(&self, duration: Duration);
}
