use core::marker::PhantomData;

use crate::multi::traits::chain_at::{HasChainTypeAt, ProvideChainTypeAt};
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

pub type SelectRelayAToB = SelectRelayChains<Index<0>, Index<1>>;

pub type SelectRelayBToA = SelectRelayChains<Index<1>, Index<0>>;
