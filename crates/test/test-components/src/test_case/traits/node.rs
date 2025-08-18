use hermes_prelude::*;

#[cgp_component {
  provider: FullNodeHalter,
  context: ChainDriver,
}]
#[async_trait]
pub trait CanHaltFullNode: HasAsyncErrorType {
    async fn halt_full_node(&self) -> Result<(), Self::Error>;
}

#[cgp_component {
  provider: FullNodeResumer,
  context: ChainDriver,
}]
#[async_trait]
pub trait CanResumeFullNode: HasAsyncErrorType + Async + Sized {
    async fn resume_full_node(&self) -> Result<Self, Self::Error>;
}
