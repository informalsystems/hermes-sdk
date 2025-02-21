use core::time::Duration;

use cgp::prelude::*;
use hermes_relayer_components::transaction::impls::poll_tx_response::{
    PollTimeoutGetter, PollTimeoutGetterComponent,
};

pub struct FixedPollTimeoutMillis<const MILLIS: u64>;

#[cgp_provider(PollTimeoutGetterComponent)]
impl<Context, const MILLIS: u64> PollTimeoutGetter<Context> for FixedPollTimeoutMillis<MILLIS> {
    fn poll_timeout(_context: &Context) -> Duration {
        Duration::from_millis(MILLIS)
    }
}
