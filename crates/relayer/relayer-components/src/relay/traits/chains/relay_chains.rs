use core::marker::PhantomData;

use cgp::core::macros::trait_alias;

use crate::multi::traits::chain_at::HasChainAt;
use crate::multi::types::tags::{Dst, Src};
use crate::relay::traits::chains::types::{HasDstChainType, HasRelayChainTypes, HasSrcChainType};

#[trait_alias]
pub trait HasSrcChain: HasSrcChainType + HasChainAt<Src> {
    fn src_chain(&self) -> &Self::SrcChain {
        self.chain_at(PhantomData)
    }
}

#[trait_alias]
pub trait HasDstChain: HasDstChainType + HasChainAt<Dst> {
    fn dst_chain(&self) -> &Self::DstChain {
        self.chain_at(PhantomData)
    }
}

#[trait_alias]
pub trait HasRelayChains: HasRelayChainTypes + HasSrcChain + HasDstChain {}
