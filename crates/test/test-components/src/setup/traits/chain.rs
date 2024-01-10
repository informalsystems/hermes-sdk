use cgp_core::prelude::*;

use crate::driver::traits::types::chain_at::{ChainTypeAt, HasChainTypeAt};
use crate::types::index::Index;

#[derive_component(ChainSetupComponent, ChainSetup<Setup>)]
#[async_trait]
pub trait CanSetupChain<const I: usize>: HasChainTypeAt<I> + HasErrorType {
    async fn setup_chain(&self, index: Index<I>) -> Result<ChainTypeAt<Self, I>, Self::Error>;
}
