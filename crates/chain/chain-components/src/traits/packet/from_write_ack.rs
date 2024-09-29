use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::packet::HasOutgoingPacketType;

use crate::traits::types::ibc_events::write_ack::HasWriteAckEvent;

#[derive_component(PacketFromWriteAckBuilderComponent, PacketFromWriteAckBuilder<Chain>)]
pub trait CanBuildPacketFromWriteAck<Counterparty>: HasWriteAckEvent<Counterparty>
where
    Counterparty: HasOutgoingPacketType<Self>,
{
    /**
       Extract the [`IncomingPacket`](HasIbcPacketTypes::IncomingPacket)
       from a write acknowledgement event.

       Since write acknowledgements are emitted from a destination chain (self),
       it is necessary for the event to correspond to an incoming packet
       (with self being the destination).

       Here we assume that a write acknowledgement event always contains
       the packet data. This is currently true for Cosmos chains. However
       in case additional queries are required, then this method should be
       refactored into a method like
       `query_packet_from_write_ack_event`.
    */
    fn build_packet_from_write_ack_event(
        ack: &Self::WriteAckEvent,
    ) -> &Counterparty::OutgoingPacket;
}
