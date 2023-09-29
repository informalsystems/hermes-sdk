use core::time::Duration;

use cgp_async::async_generic_trait;
use cgp_core::traits::Async;

use crate::std_prelude::*;

#[async_generic_trait]
pub trait CanSleep: Async {
    async fn sleep(&self, duration: Duration);
}
