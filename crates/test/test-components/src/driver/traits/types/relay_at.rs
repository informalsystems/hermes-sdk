use cgp_core::prelude::*;

use hermes_relayer_components::relay::traits::chains::HasRelayChains;

use crate::driver::traits::types::chain_at::{ChainTypeAt, HasChainTypeAt};

#[derive_component(RelayTypeAtComponent, ProvideRelayTypeAt<Context>)]
pub trait HasRelayTypeAt<const SRC: usize, const DST: usize>:
    HasChainTypeAt<SRC> + HasChainTypeAt<DST>
{
    type Relay: HasRelayChains<SrcChain = ChainTypeAt<Self, SRC>, DstChain = ChainTypeAt<Self, DST>>;
}

pub type RelayTypeAt<Context, const SRC: usize, const DST: usize> =
    <Context as HasRelayTypeAt<SRC, DST>>::Relay;
