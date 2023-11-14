use ibc_relayer_components::chain::traits::types::chain_id::{
    ChainIdGetter, ChainIdTypeProvider, HasChainId, HasChainIdType,
};

use crate::traits::inner::HasInner;

pub struct ForwardInnerChainId;

impl<Chain, Inner> ChainIdTypeProvider<Chain> for ForwardInnerChainId
where
    Chain: HasInner<Inner = Inner>,
    Inner: HasChainIdType,
{
    type ChainId = Inner::ChainId;
}

impl<Chain, Inner> ChainIdGetter<Chain> for ForwardInnerChainId
where
    Chain: HasInner<Inner = Inner> + HasChainIdType,
    Inner: HasChainId<ChainId = Chain::ChainId>,
{
    fn chain_id(chain: &Chain) -> &Chain::ChainId {
        chain.inner().chain_id()
    }
}
