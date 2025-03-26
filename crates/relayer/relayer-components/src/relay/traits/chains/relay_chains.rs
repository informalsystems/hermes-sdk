use core::marker::PhantomData;

use cgp::core::macros::blanket_trait;

use crate::multi::traits::chain_at::HasChainAt;
use crate::multi::types::tags::{Dst, Src};
use crate::relay::traits::chains::types::{HasDstChainType, HasRelayChainTypes, HasSrcChainType};

#[blanket_trait]
pub trait HasSrcChain: HasSrcChainType + HasChainAt<Src> {
    fn src_chain(&self) -> &Self::SrcChain {
        self.chain_at(PhantomData)
    }
}

#[blanket_trait]
pub trait HasDstChain: HasDstChainType + HasChainAt<Dst> {
    fn dst_chain(&self) -> &Self::DstChain {
        self.chain_at(PhantomData)
    }
}

#[blanket_trait]
pub trait HasRelayChains: HasRelayChainTypes + HasSrcChain + HasDstChain {}
