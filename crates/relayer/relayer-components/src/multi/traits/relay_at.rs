use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::ClientIdOf;
use crate::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use crate::relay::traits::chains::HasRelayChains;

#[derive_component(RelayTypeAtComponent, ProvideRelayTypeAt<Context>)]
pub trait HasRelayTypeAt<const SRC: usize, const DST: usize>: Async {
    type Relay: Async;
}

pub type RelayAt<Context, const SRC: usize, const DST: usize> =
    <Context as HasRelayTypeAt<SRC, DST>>::Relay;

pub type ClientIdAt<Context, const SRC: usize, const DST: usize> =
    ClientIdOf<ChainAt<Context, SRC>, ChainAt<Context, DST>>;

pub trait HasBoundedRelayTypeAt<const SRC: usize, const DST: usize>:
    HasRelayTypeAt<
        SRC,
        DST,
        Relay: HasRelayChains<SrcChain = ChainAt<Self, SRC>, DstChain = ChainAt<Self, DST>>,
    > + HasChainTypeAt<SRC, Chain: HasIbcChainTypes<ChainAt<Self, DST>>>
    + HasChainTypeAt<DST, Chain: HasIbcChainTypes<ChainAt<Self, SRC>>>
{
}

impl<Context, const SRC: usize, const DST: usize> HasBoundedRelayTypeAt<SRC, DST> for Context where
    Context: HasRelayTypeAt<
            SRC,
            DST,
            Relay: HasRelayChains<SrcChain = ChainAt<Self, SRC>, DstChain = ChainAt<Self, DST>>,
        > + HasChainTypeAt<SRC, Chain: HasIbcChainTypes<ChainAt<Self, DST>>>
        + HasChainTypeAt<DST, Chain: HasIbcChainTypes<ChainAt<Self, SRC>>>
{
}

impl<Context, const SRC: usize, const DST: usize, Provider, Relay>
    ProvideRelayTypeAt<Context, SRC, DST> for WithProvider<Provider>
where
    Context: Async,
    Provider: ProvideType<Context, RelayTypeAtComponent, Type = Relay>,
    Relay: Async,
{
    type Relay = Relay;
}
