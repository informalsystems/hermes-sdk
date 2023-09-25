use core::time::Duration;

use async_trait::async_trait;
use cgp_core::traits::Async;

use crate::std_prelude::*;

#[async_trait]
pub trait CanSleep: Async {
    async fn sleep(&self, duration: Duration);
}
