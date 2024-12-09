use cgp::prelude::*;

#[cgp_component {
    context: ChainDriver,
    provider: ChainStartupWaiter,
}]
#[async_trait]
pub trait CanWaitChainStartup: Async + HasErrorType {
    async fn wait_chain_startup(&self) -> Result<(), Self::Error>;
}
