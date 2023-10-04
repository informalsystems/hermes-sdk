use core::time::Duration;

use cgp_core::{async_trait, Async};

#[allow(unused_imports)]
use crate::std_prelude::*;

#[async_trait]
pub trait CanSleep: Async {
    async fn sleep(&self, duration: Duration);
}
