use cgp_core::HasInner;

use crate::chain::traits::types::chain_id::{
    ChainIdGetter, HasChainId, HasChainIdType, ProvideChainIdType,
};

pub struct ForwardChainId;

impl<Chain, Inner> ProvideChainIdType<Chain> for ForwardChainId
where
    Chain: HasInner<Inner = Inner>,
    Inner: HasChainIdType,
{
    type ChainId = Inner::ChainId;
}

impl<Chain, Inner> ChainIdGetter<Chain> for ForwardChainId
where
    Chain: HasInner<Inner = Inner> + HasChainIdType,
    Inner: HasChainId<ChainId = Chain::ChainId>,
{
    fn chain_id(chain: &Chain) -> &Chain::ChainId {
        chain.inner().chain_id()
    }
}
