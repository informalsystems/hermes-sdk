use alloc::format;
use alloc::string::String;

use hermes_prelude::*;
use hermes_runtime_components::traits::{CanSleep, HasRuntime};

use crate::traits::{CanQueryBlockTime, CanQueryChainHeight, HasHeightType};

#[async_trait]
pub trait CanWaitChainReachHeight: HasHeightType + HasAsyncErrorType {
    async fn wait_chain_reach_height(
        &self,
        height: &Self::Height,
    ) -> Result<Self::Height, Self::Error>;
}

impl<Chain> CanWaitChainReachHeight for Chain
where
    Chain: CanQueryChainHeight + HasRuntime + CanQueryBlockTime + CanRaiseAsyncError<String>,
    Chain::Runtime: CanSleep,
    Chain::Height: Clone,
{
    async fn wait_chain_reach_height(
        &self,
        height: &Chain::Height,
    ) -> Result<Chain::Height, Chain::Error> {
        let block_time = self.query_block_time().await?;
        // Wait at maximum 1 minute
        for _ in 0..600 {
            let current_height = self.query_chain_height().await?;

            if &current_height >= height {
                return Ok(current_height.clone());
            } else {
                self.runtime().sleep(block_time).await;
            }
        }
        let current_height = self.query_chain_height().await?;
        Err(Chain::raise_error(format!("chain failed to reached desired height `{height:?}` after 1 minute. Current chain height `{current_height:?}`")))
    }
}
