use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::ClientIdOf;
use crate::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use crate::multi::types::tags::{Dst, Src};
use crate::relay::traits::chains::HasRelayChainTypes;

#[derive_component(RelayTypeAtComponent<SrcTag, DstTag>, ProvideRelayTypeAt<Context>)]
pub trait HasRelayTypeAt<SrcTag, DstTag>: Async {
    type Relay: Async;
}

pub type RelayAt<Context, SrcTag, DstTag> = <Context as HasRelayTypeAt<SrcTag, DstTag>>::Relay;

pub type ClientIdAt<Context, SrcTag, DstTag> =
    ClientIdOf<ChainAt<Context, SrcTag>, ChainAt<Context, DstTag>>;

pub trait HasBoundedRelayTypeAt<SrcTag, DstTag>:
    HasRelayTypeAt<
        SrcTag,
        DstTag,
        Relay: HasChainTypeAt<Src, Chain = ChainAt<Self, SrcTag>>
                   + HasChainTypeAt<Dst, Chain = ChainAt<Self, DstTag>>
                   + HasRelayChainTypes<
            SrcChain = ChainAt<Self, SrcTag>,
            DstChain = ChainAt<Self, DstTag>,
        >,
    > + HasChainTypeAt<SrcTag, Chain: HasIbcChainTypes<ChainAt<Self, DstTag>>>
    + HasChainTypeAt<DstTag, Chain: HasIbcChainTypes<ChainAt<Self, SrcTag>>>
{
}

impl<Context, SrcTag, DstTag> HasBoundedRelayTypeAt<SrcTag, DstTag> for Context where
    Context: HasRelayTypeAt<
            SrcTag,
            DstTag,
            Relay: HasRelayChainTypes<
                SrcChain = ChainAt<Self, SrcTag>,
                DstChain = ChainAt<Self, DstTag>,
            >,
        > + HasChainTypeAt<SrcTag, Chain: HasIbcChainTypes<ChainAt<Self, DstTag>>>
        + HasChainTypeAt<DstTag, Chain: HasIbcChainTypes<ChainAt<Self, SrcTag>>>
{
}

impl<Context, SrcTag, DstTag, Provider, Relay> ProvideRelayTypeAt<Context, SrcTag, DstTag>
    for WithProvider<Provider>
where
    Context: Async,
    Provider: ProvideType<Context, RelayTypeAtComponent<SrcTag, DstTag>, Type = Relay>,
    Relay: Async,
{
    type Relay = Relay;
}
