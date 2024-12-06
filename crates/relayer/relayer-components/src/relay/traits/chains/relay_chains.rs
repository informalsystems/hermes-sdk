use core::marker::PhantomData;

use crate::multi::traits::chain_at::HasChainAt;
use crate::multi::types::tags::{Dst, Src};
use crate::relay::traits::chains::types::{HasDstChainType, HasRelayChainTypes, HasSrcChainType};

pub trait HasSrcChain: HasSrcChainType + HasChainAt<Src> {
    fn src_chain(&self) -> &Self::SrcChain;
}

pub trait HasDstChain: HasDstChainType + HasChainAt<Dst> {
    fn dst_chain(&self) -> &Self::DstChain;
}

impl<Relay> HasSrcChain for Relay
where
    Relay: HasChainAt<Src>,
{
    fn src_chain(&self) -> &Self::SrcChain {
        self.chain_at(PhantomData)
    }
}

impl<Relay> HasDstChain for Relay
where
    Relay: HasChainAt<Dst>,
{
    fn dst_chain(&self) -> &Self::DstChain {
        self.chain_at(PhantomData)
    }
}

pub trait HasRelayChains: HasRelayChainTypes + HasSrcChain + HasDstChain {}

impl<Relay> HasRelayChains for Relay where Relay: HasRelayChainTypes + HasSrcChain + HasDstChain {}
