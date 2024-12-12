use core::marker::PhantomData;

use crate::multi::traits::chain_at::{
    ChainGetterAt, HasChainAt, HasChainTypeAt, ProvideChainTypeAt,
};
use crate::multi::types::index::Index;
use crate::multi::types::tags::{Dst, Src};

pub struct SelectRelayChains<SrcTag, DstTag>(pub PhantomData<(SrcTag, DstTag)>);

impl<Relay, SrcTag, DstTag> ProvideChainTypeAt<Relay, Src> for SelectRelayChains<SrcTag, DstTag>
where
    Relay: HasChainTypeAt<SrcTag>,
{
    type Chain = Relay::Chain;
}

impl<Relay, SrcTag, DstTag> ProvideChainTypeAt<Relay, Dst> for SelectRelayChains<SrcTag, DstTag>
where
    Relay: HasChainTypeAt<DstTag>,
{
    type Chain = Relay::Chain;
}

impl<Relay, SrcTag, DstTag, Chain> ChainGetterAt<Relay, Src> for SelectRelayChains<SrcTag, DstTag>
where
    Relay: HasChainAt<SrcTag, Chain = Chain> + HasChainTypeAt<Src, Chain = Chain>,
{
    fn chain_at(relay: &Relay, _tag: PhantomData<Src>) -> &Chain {
        relay.chain_at(PhantomData::<SrcTag>)
    }
}

impl<Relay, SrcTag, DstTag, Chain> ChainGetterAt<Relay, Dst> for SelectRelayChains<SrcTag, DstTag>
where
    Relay: HasChainAt<DstTag, Chain = Chain> + HasChainTypeAt<Dst, Chain = Chain>,
{
    fn chain_at(relay: &Relay, _tag: PhantomData<Dst>) -> &Chain {
        relay.chain_at(PhantomData::<DstTag>)
    }
}
pub type SelectRelayAToB = SelectRelayChains<Index<0>, Index<1>>;

pub type SelectRelayBToA = SelectRelayChains<Index<1>, Index<0>>;
