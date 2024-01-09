use cgp_core::prelude::*;

use crate::driver::traits::types::chain_at::{ChainTypeAt, HasChainTypeAt};

#[async_trait]
pub trait CanSetupBinaryChain: HasChainTypeAt<0> + HasChainTypeAt<1> + HasErrorType {
    async fn setup_binary_chain(
        &self,
    ) -> Result<(ChainTypeAt<Self, 0>, ChainTypeAt<Self, 1>), Self::Error>;
}
