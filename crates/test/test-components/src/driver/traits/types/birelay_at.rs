use hermes_relayer_components::birelay::traits::two_way::HasTwoWayRelayTypes;

use crate::driver::traits::types::chain_at::ChainTypeAt;
use crate::driver::traits::types::relay_at::{HasRelayTypeAt, RelayTypeAt};

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
