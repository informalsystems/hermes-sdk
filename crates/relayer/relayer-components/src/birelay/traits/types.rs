use cgp::core::field::Index;
use cgp::core::macros::trait_alias;

use crate::multi::traits::chain_at::HasChainTypeAt;
use crate::multi::traits::relay_at::HasRelayTypeAt;
use crate::relay::traits::chains::{HasDstChainType, HasSrcChainType};

#[trait_alias]
pub trait HasBiRelayTypes:
    HasChainTypeAt<Index<0>, Chain = Self::ChainA>
    + HasChainTypeAt<Index<1>, Chain = Self::ChainB>
    + HasRelayTypeAt<Index<0>, Index<1>, Relay = Self::RelayAToB>
    + HasRelayTypeAt<Index<1>, Index<0>, Relay = Self::RelayBToA>
{
    type RelayAToB: HasSrcChainType<SrcChain = Self::ChainA>
        + HasDstChainType<DstChain = Self::ChainB>;

    type RelayBToA: HasSrcChainType<SrcChain = Self::ChainB>
        + HasDstChainType<DstChain = Self::ChainA>;

    type ChainA;

    type ChainB;
}
