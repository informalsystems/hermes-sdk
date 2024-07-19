use cgp_core::prelude::*;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::ClientIdOf;
use crate::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use crate::relay::traits::chains::HasRelayChains;

#[derive_component(RelayTypeAtComponent, ProvideRelayTypeAt<Context>)]
pub trait HasRelayTypeAt<const SRC: usize, const DST: usize>:
    HasChainTypeAt<SRC, Chain: HasIbcChainTypes<ChainAt<Self, DST>>>
    + HasChainTypeAt<DST, Chain: HasIbcChainTypes<ChainAt<Self, SRC>>>
{
    type Relay: HasRelayChains<SrcChain = ChainAt<Self, SRC>, DstChain = ChainAt<Self, DST>>;
}

pub type RelayAt<Context, const SRC: usize, const DST: usize> =
    <Context as HasRelayTypeAt<SRC, DST>>::Relay;

pub type ClientIdAt<Context, const SRC: usize, const DST: usize> =
    ClientIdOf<ChainAt<Context, SRC>, ChainAt<Context, DST>>;
