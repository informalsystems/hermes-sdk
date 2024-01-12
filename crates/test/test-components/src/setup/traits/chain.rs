use cgp_core::prelude::*;

use crate::driver::traits::types::chain_at::{ChainTypeAt, HasChainTypeAt};
use crate::driver::traits::types::chain_driver_at::{ChainDriverTypeAt, HasChainDriverTypeAt};
use crate::types::index::Index;

#[derive_component(ChainSetupComponent, ChainSetup<Setup>)]
#[async_trait]
pub trait CanSetupChain<const I: usize>: HasChainDriverTypeAt<I> + HasErrorType {
    async fn setup_chain(&self, index: Index<I>)
        -> Result<ChainDriverTypeAt<Self, I>, Self::Error>;
}
