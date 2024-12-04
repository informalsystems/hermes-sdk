use cgp::prelude::HasErrorType;
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::multi::traits::chain_at::HasChainTypeAt;
use crate::multi::types::tags::{Dst, Src};

pub trait HasSrcChainType: HasChainTypeAt<Src, Chain = Self::SrcChain> {
    type SrcChain;
}

pub trait HasDstChainType: HasChainTypeAt<Dst, Chain = Self::DstChain> {
    type DstChain;
}

impl<Relay> HasSrcChainType for Relay
where
    Relay: HasChainTypeAt<Src>,
{
    type SrcChain = Relay::Chain;
}

impl<Relay> HasDstChainType for Relay
where
    Relay: HasChainTypeAt<Dst>,
{
    type DstChain = Relay::Chain;
}

pub trait HasRelayChainTypes:
    HasErrorType
    + HasSrcChainType<
        SrcChain: HasErrorType
                      + HasIbcChainTypes<Self::DstChain>
                      + HasOutgoingPacketType<Self::DstChain>,
    > + HasDstChainType<DstChain: HasErrorType + HasIbcChainTypes<Self::SrcChain>>
{
}

impl<Relay, SrcChain, DstChain> HasRelayChainTypes for Relay
where
    Relay: HasChainTypeAt<Src, Chain = SrcChain>
        + HasChainTypeAt<Dst, Chain = DstChain>
        + HasErrorType,
    SrcChain: HasErrorType + HasIbcChainTypes<DstChain> + HasOutgoingPacketType<DstChain>,
    DstChain: HasErrorType + HasIbcChainTypes<SrcChain>,
{
}

pub type SrcChainOf<Relay> = <Relay as HasSrcChainType>::SrcChain;

pub type DstChainOf<Relay> = <Relay as HasDstChainType>::DstChain;

pub type PacketOf<Relay> =
    <SrcChainOf<Relay> as HasOutgoingPacketType<DstChainOf<Relay>>>::OutgoingPacket;
