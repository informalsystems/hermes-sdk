use cgp::prelude::*;

#[cgp_component {
    provider: AutoBiRelayer,
    context: BiRelay,
}]
#[async_trait]
pub trait CanAutoBiRelay: HasAsyncErrorType {
    async fn auto_bi_relay(
        &self,
        clear_past_blocks: Option<u64>,
        stop_after_blocks: Option<u64>,
    ) -> Result<(), Self::Error>;
}
