use core::time::Duration;

use cgp_core::prelude::*;

#[derive_component(PollAssertDurationGetterComponent, PollAssertDurationGetter<ChainDriver>)]
pub trait HasPollAssertDuration: Async {
    fn poll_assert_interval(&self) -> Duration;

    fn poll_assert_attempts(&self) -> u32;
}
