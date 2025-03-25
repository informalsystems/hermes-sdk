use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::macros::blanket_trait;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::ClientIdOf;
use crate::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use crate::multi::types::tags::{Dst, Src};
use crate::relay::traits::chains::HasRelayChainTypes;

#[cgp_type {
    name: RelayTypeProviderAtComponent<A, B>,
    provider: RelayTypeProviderAt,
}]
pub trait HasRelayTypeAt<A, B>: Async {
    type Relay: Async;
}

#[cgp_getter {
    name: RelayGetterAtComponent<A, B>,
    provider: RelayGetterAt,
}]
pub trait HasRelayAt<A, B>: HasRelayTypeAt<A, B> {
    fn relay_at(&self, _phantom: PhantomData<(A, B)>) -> &Self::Relay;
}

pub type RelayAt<Context, SrcTag, DstTag> = <Context as HasRelayTypeAt<SrcTag, DstTag>>::Relay;

pub type ClientIdAt<Context, SrcTag, DstTag> =
    ClientIdOf<ChainAt<Context, SrcTag>, ChainAt<Context, DstTag>>;

#[blanket_trait]
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
