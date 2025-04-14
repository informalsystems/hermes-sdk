use core::time::Duration;

use cgp::prelude::*;

use crate::chain::traits::{PollAssertDurationGetter, PollAssertDurationGetterComponent};

pub struct ProvidePollAssertDuration<const INTERVAL: u64, const ATTEMPTS: u32>;

#[cgp_provider(PollAssertDurationGetterComponent)]
impl<Chain, const INTERVAL: u64, const ATTEMPTS: u32> PollAssertDurationGetter<Chain>
    for ProvidePollAssertDuration<INTERVAL, ATTEMPTS>
where
    Chain: Async,
{
    fn poll_assert_interval(_chain: &Chain) -> Duration {
        Duration::from_secs(INTERVAL)
    }

    fn poll_assert_attempts(_chain: &Chain) -> u32 {
        ATTEMPTS
    }
}
