use hermes_prelude::*;

use crate::traits::{HasBlockType, HasHeightType};

#[cgp_component {
  provider: BlockQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryBlock: HasHeightType + HasBlockType + HasAsyncErrorType {
    async fn query_block(&self, height: &Self::Height) -> Result<Self::Block, Self::Error>;
}
