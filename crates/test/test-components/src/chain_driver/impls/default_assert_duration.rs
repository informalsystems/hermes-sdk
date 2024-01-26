use core::time::Duration;

use cgp_core::prelude::Async;

use crate::chain_driver::traits::assert::poll_assert::PollAssertDurationGetter;

pub struct ProvideDefaultPollAssertDuration;

impl<ChainDriver> PollAssertDurationGetter<ChainDriver> for ProvideDefaultPollAssertDuration
where
    ChainDriver: Async,
{
    fn poll_assert_interval(_chain_driver: &ChainDriver) -> Duration {
        Duration::from_secs(1)
    }

    fn poll_assert_attempts(_chain_driver: &ChainDriver) -> u32 {
        90
    }
}
