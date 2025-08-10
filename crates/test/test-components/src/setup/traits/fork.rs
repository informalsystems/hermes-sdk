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
