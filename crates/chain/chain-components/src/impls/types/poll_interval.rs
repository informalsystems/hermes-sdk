use core::time::Duration;

use cgp::prelude::*;

use crate::traits::types::poll_interval::{PollIntervalGetter, PollIntervalGetterComponent};

pub struct FixedPollIntervalMillis<const MILLIS: u64>;

#[cgp_provider(PollIntervalGetterComponent)]
impl<Context, const MILLIS: u64> PollIntervalGetter<Context> for FixedPollIntervalMillis<MILLIS> {
    fn poll_interval(_context: &Context) -> Duration {
        Duration::from_millis(MILLIS)
    }
}
