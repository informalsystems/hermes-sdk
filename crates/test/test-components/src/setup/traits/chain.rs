use cgp::prelude::*;
use hermes_relayer_components::multi::types::index::Index;

use crate::driver::traits::types::chain_driver_at::{ChainDriverTypeAt, HasChainDriverTypeAt};

#[derive_component(ChainSetupComponent, ChainSetup<Setup>)]
#[async_trait]
pub trait CanSetupChain<const I: usize>: HasChainDriverTypeAt<I> + HasErrorType {
    async fn setup_chain(&self, index: Index<I>)
        -> Result<ChainDriverTypeAt<Self, I>, Self::Error>;
}
