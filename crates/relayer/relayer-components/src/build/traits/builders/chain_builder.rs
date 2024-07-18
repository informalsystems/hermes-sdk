use cgp_core::prelude::*;

use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::chain::types::aliases::ChainIdOf;
use crate::multi::traits::chain_at::HasChainTypeAt;
use crate::multi::types::index::Index;

#[derive_component(ChainBuilderComponent, ChainBuilder<Build>)]
#[async_trait]
pub trait CanBuildChain<const TARGET: usize>:
    HasChainTypeAt<TARGET, Chain: HasChainIdType> + HasErrorType
{
    async fn build_chain(
        &self,
        index: Index<TARGET>,
        chain_id: &ChainIdOf<Self::Chain>,
    ) -> Result<Self::Chain, Self::Error>;
}
