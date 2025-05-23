use cgp::core::macros::blanket_trait;
use hermes_chain_components::traits::HasOutgoingPacketType;
use hermes_prelude::{HasAsyncErrorType, HasErrorType};

use crate::chain::traits::HasIbcChainTypes;
use crate::multi::traits::chain_at::HasChainTypeAt;
use crate::multi::types::tags::{Dst, Src};

#[blanket_trait]
pub trait HasSrcChainType: HasChainTypeAt<Src, Chain = Self::SrcChain> {
    type SrcChain;
}

#[blanket_trait]
pub trait HasDstChainType: HasChainTypeAt<Dst, Chain = Self::DstChain> {
    type DstChain;
}

#[blanket_trait]
pub trait HasRelayChainTypes:
    HasAsyncErrorType
    + HasSrcChainType<
        SrcChain: HasErrorType
                      + HasIbcChainTypes<Self::DstChain>
                      + HasOutgoingPacketType<Self::DstChain>,
    > + HasDstChainType<DstChain: HasErrorType + HasIbcChainTypes<Self::SrcChain>>
{
}

pub type SrcChainOf<Relay> = <Relay as HasSrcChainType>::SrcChain;

pub type DstChainOf<Relay> = <Relay as HasDstChainType>::DstChain;
