use cgp_core::prelude::*;

use crate::chain::traits::types::block::HasBlockType;
use crate::chain::traits::types::height::HasHeightType;

#[derive_component(BlockQuerierComponent, BlockQuerier<Chain>)]
#[async_trait]
pub trait CanQueryBlock: HasHeightType + HasBlockType + HasErrorType {
    async fn query_block(&self, height: &Self::Height) -> Result<Self::Block, Self::Error>;
}
