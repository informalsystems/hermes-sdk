use core::time::Duration;

use cgp::prelude::*;
use hermes_relayer_components::transaction::impls::poll_tx_response::{
    PollTimeoutGetter, PollTimeoutGetterComponent,
};

pub struct DefaultPollTimeout;

#[cgp_provider(PollTimeoutGetterComponent)]
impl<Chain> PollTimeoutGetter<Chain> for DefaultPollTimeout {
    fn poll_timeout(_chain: &Chain) -> Duration {
        Duration::from_secs(300)
    }

    fn poll_backoff(_chain: &Chain) -> Duration {
        Duration::from_millis(100)
    }
}
