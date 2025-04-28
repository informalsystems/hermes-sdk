use hermes_prelude::*;

#[cgp_component {
    context: ChainDriver,
    provider: ChainStartupWaiter,
}]
#[async_trait]
pub trait CanWaitChainStartup: Async + HasAsyncErrorType {
    async fn wait_chain_startup(&self) -> Result<(), Self::Error>;
}
