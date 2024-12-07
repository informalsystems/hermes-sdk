use core::time::Duration;

use cgp::prelude::*;

#[cgp_component {
  name: PollAssertDurationGetterComponent,
  provider: PollAssertDurationGetter,
  context: ChainDriver,
}]
pub trait HasPollAssertDuration: Async {
    fn poll_assert_interval(&self) -> Duration;

    fn poll_assert_attempts(&self) -> u32;
}
