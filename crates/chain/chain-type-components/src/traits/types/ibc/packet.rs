/*!
   Trait definition for [`HasIbcPacketTypes`].
*/

use cgp::prelude::*;

#[derive_component(OutgoingPacketTypeComponent, ProvideOutgoingPacketType<Chain>)]
pub trait HasOutgoingPacketType<Counterparty>: Async {
    /**
       A packet sent from self to counterparty.

       - Packet source: `Self`
       - Packet destination: `Counterparty`
    */
    type OutgoingPacket: Async;
}
