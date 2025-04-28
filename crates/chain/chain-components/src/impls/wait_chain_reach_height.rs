use core::time::Duration;

use hermes_prelude::*;
use hermes_runtime_components::traits::{CanSleep, HasRuntime};

use crate::traits::{CanQueryChainHeight, HasHeightType};

#[async_trait]
pub trait CanWaitChainReachHeight: HasHeightType + HasAsyncErrorType {
    async fn wait_chain_reach_height(
        &self,
        height: &Self::Height,
    ) -> Result<Self::Height, Self::Error>;
}

impl<Chain> CanWaitChainReachHeight for Chain
where
    Chain: CanQueryChainHeight + HasRuntime,
    Chain::Runtime: CanSleep,
    Chain::Height: Clone,
{
    async fn wait_chain_reach_height(
        &self,
        height: &Chain::Height,
    ) -> Result<Chain::Height, Chain::Error> {
        loop {
            let current_height = self.query_chain_height().await?;

            if &current_height >= height {
                return Ok(current_height.clone());
            } else {
                self.runtime().sleep(Duration::from_millis(100)).await;
            }
        }
    }
}
