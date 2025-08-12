use alloc::string::String;

use hermes_prelude::*;

#[cgp_component {
  provider: FullNodeForker,
  context: Driver,
}]
#[async_trait]
pub trait CanForkFullNode: HasAsyncErrorType + Async + Sized {
    async fn fork_full_node(&self, chain_id: String) -> Result<Self, Self::Error>;
}

#[cgp_component {
  provider: FullNodeHalter,
  context: Driver,
}]
#[async_trait]
pub trait CanHaltFullNode: HasAsyncErrorType + Async + Sized {
    async fn halt_full_node(&self, chain_id: String) -> Result<(), Self::Error>;
}

#[cgp_component {
  provider: FullNodeResumer,
  context: Driver,
}]
#[async_trait]
pub trait CanResumeFullNode: HasAsyncErrorType + Async + Sized {
    async fn resume_full_node(&self, chain_id: String) -> Result<Self, Self::Error>;
}
