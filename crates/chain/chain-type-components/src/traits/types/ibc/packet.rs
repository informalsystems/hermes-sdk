/*!
   Trait definition for [`HasIbcPacketTypes`].
*/

use cgp::prelude::*;

#[cgp_component {
  name: OutgoingPacketTypeComponent,
  provider: ProvideOutgoingPacketType,
  context: Chain,
}]
pub trait HasOutgoingPacketType<Counterparty>: Async {
    /**
       A packet sent from self to counterparty.

       - Packet source: `Self`
       - Packet destination: `Counterparty`
    */
    type OutgoingPacket: Async;
}
