use cgp::core::component::WithProvider;
use cgp::core::macros::trait_alias;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::ClientIdOf;
use crate::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use crate::multi::types::tags::{Dst, Src};
use crate::relay::traits::chains::HasRelayChainTypes;

#[cgp_type {
    name: RelayTypeProviderAtComponent<SrcTag, DstTag>,
    provider: RelayTypeProviderAt,
}]
pub trait HasRelayTypeAt<SrcTag, DstTag>: Async {
    type Relay: Async;
}

pub type RelayAt<Context, SrcTag, DstTag> = <Context as HasRelayTypeAt<SrcTag, DstTag>>::Relay;

pub type ClientIdAt<Context, SrcTag, DstTag> =
    ClientIdOf<ChainAt<Context, SrcTag>, ChainAt<Context, DstTag>>;

#[trait_alias]
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
    > + HasChainTypeAt<SrcTag, Chain: HasIbcChainTypes<ChainAt<Self, DstTag>> + HasAsyncErrorType>
    + HasChainTypeAt<DstTag, Chain: HasIbcChainTypes<ChainAt<Self, SrcTag>> + HasAsyncErrorType>
{
}
