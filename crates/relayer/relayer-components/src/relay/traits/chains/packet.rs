use cgp::core::macros::blanket_trait;
use cgp::core::Async;
use hermes_chain_components::traits::types::packet::HasOutgoingPacketType;

use crate::relay::traits::chains::{DstChainOf, HasRelayChains, SrcChainOf};

pub type PacketOf<Relay> =
    <SrcChainOf<Relay> as HasOutgoingPacketType<DstChainOf<Relay>>>::OutgoingPacket;

#[blanket_trait]
pub trait HasRelayPacketType:
    HasRelayChains<SrcChain: HasOutgoingPacketType<Self::DstChain, OutgoingPacket = Self::Packet>>
{
    type Packet: Async;
}
