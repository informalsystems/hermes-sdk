use core::time::Duration;

use cgp::prelude::*;

use crate::chain::traits::assert::poll_assert::{
    PollAssertDurationGetter, PollAssertDurationGetterComponent,
};

pub struct ProvideDefaultPollAssertDuration;

#[cgp_provider(PollAssertDurationGetterComponent)]
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
