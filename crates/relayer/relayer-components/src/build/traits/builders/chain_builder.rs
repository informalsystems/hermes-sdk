use core::marker::PhantomData;

use cgp::prelude::*;

use crate::chain::traits::HasChainIdType;
use crate::chain::types::aliases::ChainIdOf;
use crate::multi::traits::chain_at::HasChainTypeAt;

#[cgp_component {
  provider: ChainBuilder,
  context: Build,
}]
#[async_trait]
pub trait CanBuildChain<I: Async>:
    HasChainTypeAt<I, Chain: HasChainIdType> + HasAsyncErrorType
{
    async fn build_chain(
        &self,
        _tag: PhantomData<I>,
        chain_id: &ChainIdOf<Self::Chain>,
    ) -> Result<Self::Chain, Self::Error>;
}
