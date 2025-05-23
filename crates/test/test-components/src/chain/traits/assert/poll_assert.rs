use core::time::Duration;

use hermes_prelude::*;

#[cgp_component {
  provider: PollAssertDurationGetter,
  context: ChainDriver,
}]
pub trait HasPollAssertDuration: Async {
    fn poll_assert_interval(&self) -> Duration;

    fn poll_assert_attempts(&self) -> u32;
}
