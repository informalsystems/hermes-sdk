use core::time::Duration;

use cgp::prelude::*;

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
    ) -> Result<(), Self::Error>;
}
