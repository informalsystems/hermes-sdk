use cgp::prelude::*;

use crate::traits::types::block::HasBlockType;
use crate::traits::types::height::HasHeightType;

#[derive_component(BlockQuerierComponent, BlockQuerier<Chain>)]
#[async_trait]
pub trait CanQueryBlock: HasHeightType + HasBlockType + HasErrorType {
    async fn query_block(&self, height: &Self::Height) -> Result<Self::Block, Self::Error>;
}
