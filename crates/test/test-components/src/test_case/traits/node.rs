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
pub trait CanResumeFullNode: HasResumeFullNodeOptionsType + HasAsyncErrorType + Sized {
    async fn resume_full_node(
        &self,
        options: &Self::ResumeFullNodeOptions,
    ) -> Result<Self, Self::Error>;
}

#[cgp_type]
pub trait HasResumeFullNodeOptionsType {
    type ResumeFullNodeOptions: Async;
}
