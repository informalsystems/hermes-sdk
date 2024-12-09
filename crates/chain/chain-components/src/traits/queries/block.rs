use cgp::prelude::*;

use crate::traits::types::block::HasBlockType;
use crate::traits::types::height::HasHeightType;

#[cgp_component {
  provider: BlockQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryBlock: HasHeightType + HasBlockType + HasErrorType {
    async fn query_block(&self, height: &Self::Height) -> Result<Self::Block, Self::Error>;
}
