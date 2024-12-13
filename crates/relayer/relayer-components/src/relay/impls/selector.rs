use core::marker::PhantomData;

use crate::multi::traits::chain_at::{
    ChainGetterAt, HasChainAt, HasChainTypeAt, ProvideChainTypeAt,
};
use crate::multi::traits::client_id_at::{ClientIdAtGetter, HasClientIdAt};
use crate::multi::traits::relay_at::ClientIdAt;
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

impl<Relay, SrcTag, DstTag, SrcChain, DstChain> ClientIdAtGetter<Relay, Src, Dst>
    for SelectRelayChains<SrcTag, DstTag>
where
    Relay: HasChainAt<SrcTag, Chain = SrcChain>
        + HasChainTypeAt<Src, Chain = SrcChain>
        + HasChainAt<DstTag, Chain = DstChain>
        + HasChainTypeAt<Dst, Chain = DstChain>
        + HasClientIdAt<SrcTag, DstTag>,
{
    fn client_id_at(relay: &Relay, _tag: PhantomData<(Src, Dst)>) -> &ClientIdAt<Relay, Src, Dst> {
        relay.client_id_at(PhantomData::<(SrcTag, DstTag)>)
    }
}

impl<Relay, SrcTag, DstTag, SrcChain, DstChain> ClientIdAtGetter<Relay, Dst, Src>
    for SelectRelayChains<SrcTag, DstTag>
where
    Relay: HasChainAt<SrcTag, Chain = SrcChain>
        + HasChainTypeAt<Src, Chain = SrcChain>
        + HasChainAt<DstTag, Chain = DstChain>
        + HasChainTypeAt<Dst, Chain = DstChain>
        + HasClientIdAt<DstTag, SrcTag>,
{
    fn client_id_at(relay: &Relay, _tag: PhantomData<(Dst, Src)>) -> &ClientIdAt<Relay, Dst, Src> {
        relay.client_id_at(PhantomData::<(DstTag, SrcTag)>)
    }
}

pub type SelectRelayAToB = SelectRelayChains<Index<0>, Index<1>>;

pub type SelectRelayBToA = SelectRelayChains<Index<1>, Index<0>>;
