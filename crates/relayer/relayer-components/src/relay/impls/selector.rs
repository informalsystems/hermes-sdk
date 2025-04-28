use core::marker::PhantomData;

use cgp::core::field::Index;
use hermes_prelude::*;

use crate::multi::traits::chain_at::{
    ChainGetterAt, ChainGetterAtComponent, ChainTypeProviderAt, ChainTypeProviderAtComponent,
    HasChainAt, HasChainTypeAt,
};
use crate::multi::traits::client_id_at::{
    ClientIdAtGetter, ClientIdAtGetterComponent, HasClientIdAt,
};
use crate::multi::traits::relay_at::ClientIdAt;
use crate::multi::types::tags::{Dst, Src};

pub struct SelectRelayChains<SrcTag, DstTag>(pub PhantomData<(SrcTag, DstTag)>);

#[cgp_provider(ChainTypeProviderAtComponent<Src>)]
impl<Relay, SrcTag, DstTag> ChainTypeProviderAt<Relay, Src> for SelectRelayChains<SrcTag, DstTag>
where
    Relay: HasChainTypeAt<SrcTag>,
{
    type Chain = Relay::Chain;
}

#[cgp_provider(ChainTypeProviderAtComponent<Dst>)]
impl<Relay, SrcTag, DstTag> ChainTypeProviderAt<Relay, Dst> for SelectRelayChains<SrcTag, DstTag>
where
    Relay: HasChainTypeAt<DstTag>,
{
    type Chain = Relay::Chain;
}

#[cgp_provider(ChainGetterAtComponent<Src>)]
impl<Relay, SrcTag, DstTag, Chain> ChainGetterAt<Relay, Src> for SelectRelayChains<SrcTag, DstTag>
where
    Relay: HasChainAt<SrcTag, Chain = Chain> + HasChainTypeAt<Src, Chain = Chain>,
{
    fn chain_at(relay: &Relay, _tag: PhantomData<Src>) -> &Chain {
        relay.chain_at(PhantomData::<SrcTag>)
    }
}

#[cgp_provider(ChainGetterAtComponent<Dst>)]
impl<Relay, SrcTag, DstTag, Chain> ChainGetterAt<Relay, Dst> for SelectRelayChains<SrcTag, DstTag>
where
    Relay: HasChainAt<DstTag, Chain = Chain> + HasChainTypeAt<Dst, Chain = Chain>,
{
    fn chain_at(relay: &Relay, _tag: PhantomData<Dst>) -> &Chain {
        relay.chain_at(PhantomData::<DstTag>)
    }
}

#[cgp_provider(ClientIdAtGetterComponent<Src, Dst>)]
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

#[cgp_provider(ClientIdAtGetterComponent<Dst, Src>)]
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
