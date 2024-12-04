use core::marker::PhantomData;

use cgp::prelude::*;

use crate::driver::traits::types::chain_driver_at::{ChainDriverTypeAt, HasChainDriverTypeAt};

#[derive_component(ChainSetupComponent, ChainSetup<Setup>)]
#[async_trait]
pub trait CanSetupChain<Tag: Async>: HasChainDriverTypeAt<Tag> + HasErrorType {
    async fn setup_chain(
        &self,
        _tag: PhantomData<Tag>,
    ) -> Result<ChainDriverTypeAt<Self, Tag>, Self::Error>;
}
