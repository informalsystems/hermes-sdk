use cgp::core::field::Index;

use crate::multi::traits::relay_at::HasRelayTypeAt;
use crate::relay::traits::chains::{HasDstChainType, HasSrcChainType};

pub trait HasBiRelayTypes:
    HasRelayTypeAt<Index<0>, Index<1>, Relay = Self::RelayAToB>
    + HasRelayTypeAt<Index<1>, Index<0>, Relay = Self::RelayBToA>
{
    type RelayAToB: HasSrcChainType<SrcChain = Self::ChainA>
        + HasDstChainType<DstChain = Self::ChainB>;

    type RelayBToA: HasSrcChainType<SrcChain = Self::ChainB>
        + HasDstChainType<DstChain = Self::ChainA>;

    type ChainA;

    type ChainB;
}

impl<BiRelay, RelayAToB, RelayBToA, ChainA, ChainB> HasBiRelayTypes for BiRelay
where
    BiRelay: HasRelayTypeAt<Index<0>, Index<1>, Relay = RelayAToB>
        + HasRelayTypeAt<Index<1>, Index<0>, Relay = RelayBToA>,
    RelayAToB: HasSrcChainType<SrcChain = ChainA> + HasDstChainType<DstChain = ChainB>,
    RelayBToA: HasSrcChainType<SrcChain = ChainB> + HasDstChainType<DstChain = ChainA>,
{
    type RelayAToB = RelayAToB;

    type RelayBToA = RelayBToA;

    type ChainA = ChainA;

    type ChainB = ChainB;
}
