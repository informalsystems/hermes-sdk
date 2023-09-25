use core::time::Duration;

use async_trait::async_trait;

use crate::std_prelude::*;
use cgp_core::traits::sync::Async;

#[async_trait]
pub trait CanSleep: Async {
    async fn sleep(&self, duration: Duration);
}
