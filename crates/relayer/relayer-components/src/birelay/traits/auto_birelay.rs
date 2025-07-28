use core::time::Duration;

use hermes_prelude::*;

#[cgp_component {
    provider: AutoBiRelayer,
    context: BiRelay,
}]
#[async_trait]
pub trait CanAutoBiRelay: HasAsyncErrorType {
    async fn auto_bi_relay(
        &self,
        clear_past_blocks: Option<Duration>,
        stop_after_blocks: Option<Duration>,
        refresh_rate_a_to_b: Option<Duration>,
        refresh_rate_b_to_a: Option<Duration>,
    ) -> Result<(), Self::Error>;
}
