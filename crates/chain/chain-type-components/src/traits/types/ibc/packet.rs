/*!
   Trait definition for [`HasIbcPacketTypes`].
*/

use cgp::prelude::*;

#[derive_component(IncomingPacketTypeComponent, ProvideIncomingPacketType<Chain>)]
pub trait HasIncomingPacketType<Counterparty>: Async {
    /**
       A packet sent from counterparty to self.

       - Packet source: `Counterparty`
       - Packet destination: `Self`
    */
    type IncomingPacket: Async;
}

#[derive_component(OutgoingPacketTypeComponent, ProvideOutgoingPacketType<Chain>)]
pub trait HasOutgoingPacketType<Counterparty>: Async {
    /**
       A packet sent from self to counterparty.

       - Packet source: `Self`
       - Packet destination: `Counterparty`
    */
    type OutgoingPacket: Async;
}
