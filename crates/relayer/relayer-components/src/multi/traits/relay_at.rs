use cgp_core::prelude::*;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::ClientIdOf;
use crate::multi::traits::chain_at::{ChainTypeAt, HasChainTypeAt};
use crate::relay::traits::chains::HasRelayChains;

#[derive_component(RelayTypeAtComponent, ProvideRelayTypeAt<Context>)]
pub trait HasRelayTypeAt<const SRC: usize, const DST: usize>:
    HasChainTypeAt<SRC, Chain: HasIbcChainTypes<ChainTypeAt<Self, DST>>>
    + HasChainTypeAt<DST, Chain: HasIbcChainTypes<ChainTypeAt<Self, SRC>>>
{
    type Relay: HasRelayChains<SrcChain = ChainTypeAt<Self, SRC>, DstChain = ChainTypeAt<Self, DST>>;
}

pub type RelayTypeAt<Context, const SRC: usize, const DST: usize> =
    <Context as HasRelayTypeAt<SRC, DST>>::Relay;

pub type ClientIdAt<Context, const SRC: usize, const DST: usize> =
    ClientIdOf<ChainTypeAt<Context, SRC>, ChainTypeAt<Context, DST>>;
