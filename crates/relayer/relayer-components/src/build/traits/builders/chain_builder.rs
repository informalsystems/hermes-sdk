use core::marker::PhantomData;

use cgp::prelude::*;

use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::chain::types::aliases::ChainIdOf;
use crate::multi::traits::chain_at::HasChainTypeAt;

#[derive_component(ChainBuilderComponent, ChainBuilder<Build>)]
#[async_trait]
pub trait CanBuildChain<I: Async>: HasChainTypeAt<I, Chain: HasChainIdType> + HasErrorType {
    async fn build_chain(
        &self,
        _tag: PhantomData<I>,
        chain_id: &ChainIdOf<Self::Chain>,
    ) -> Result<Self::Chain, Self::Error>;
}
