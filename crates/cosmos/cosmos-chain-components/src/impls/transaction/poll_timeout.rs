use core::time::Duration;

use cgp::prelude::*;
use hermes_relayer_components::transaction::impls::poll_tx_response::{
    PollTimeoutGetter, PollTimeoutGetterComponent,
};

pub struct FixedPollTimeoutSecs<const SECS: u64>;

#[cgp_provider(PollTimeoutGetterComponent)]
impl<Context, const SECS: u64> PollTimeoutGetter<Context> for FixedPollTimeoutSecs<SECS> {
    fn poll_timeout(_context: &Context) -> Duration {
        Duration::from_secs(SECS)
    }
}
