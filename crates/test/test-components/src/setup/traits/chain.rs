use core::marker::PhantomData;

use cgp::prelude::*;

use crate::driver::traits::types::chain_driver_at::{ChainDriverAt, HasChainDriverTypeAt};

#[cgp_component {
  provider: ChainSetup,
  context: Setup,
}]
#[async_trait]
pub trait CanSetupChain<Tag: Async>: HasChainDriverTypeAt<Tag> + HasAsyncErrorType {
    async fn setup_chain(
        &self,
        _tag: PhantomData<Tag>,
    ) -> Result<ChainDriverAt<Self, Tag>, Self::Error>;
}
