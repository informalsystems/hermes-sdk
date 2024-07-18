use cgp_core::prelude::*;

use crate::birelay::traits::two_way::HasTwoWayRelayTypes;
use crate::multi::traits::chain_at::ChainTypeAt;
use crate::multi::traits::relay_at::{HasRelayTypeAt, RelayTypeAt};

#[derive_component(BiRelayTypeAtComponent, ProvideBiRelayTypeAt<Setup>)]
pub trait HasBiRelayTypeAt<const A: usize, const B: usize>:
    HasRelayTypeAt<A, B> + HasRelayTypeAt<B, A>
{
    type BiRelay: HasTwoWayRelayTypes<
        ChainA = ChainTypeAt<Self, A>,
        ChainB = ChainTypeAt<Self, B>,
        RelayAToB = RelayTypeAt<Self, A, B>,
        RelayBToA = RelayTypeAt<Self, B, A>,
    >;
}

pub type BiRelayTypeAt<Context, const A: usize, const B: usize> =
    <Context as HasBiRelayTypeAt<A, B>>::BiRelay;
