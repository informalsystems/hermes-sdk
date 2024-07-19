use cgp_core::prelude::*;

use crate::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use crate::multi::traits::relay_at::{HasRelayTypeAt, RelayTypeAt};

#[derive_component(BiRelayTypeAtComponent, ProvideBiRelayTypeAt<Setup>)]
pub trait HasBiRelayTypeAt<const A: usize, const B: usize>:
    HasRelayTypeAt<A, B> + HasRelayTypeAt<B, A>
{
    type BiRelay: HasChainTypeAt<A, Chain = ChainAt<Self, A>>
        + HasChainTypeAt<B, Chain = ChainAt<Self, B>>
        + HasRelayTypeAt<A, B, Relay = RelayTypeAt<Self, A, B>>
        + HasRelayTypeAt<B, A, Relay = RelayTypeAt<Self, B, A>>;
}

pub type BiRelayTypeAt<Context, const A: usize, const B: usize> =
    <Context as HasBiRelayTypeAt<A, B>>::BiRelay;