use core::time::Duration;

use cgp::prelude::Async;

use crate::chain::traits::assert::poll_assert::PollAssertDurationGetter;

pub struct ProvideDefaultPollAssertDuration;

impl<Chain> PollAssertDurationGetter<Chain> for ProvideDefaultPollAssertDuration
where
    Chain: Async,
{
    fn poll_assert_interval(_chain: &Chain) -> Duration {
        Duration::from_secs(1)
    }

    fn poll_assert_attempts(_chain: &Chain) -> u32 {
        90
    }
}
