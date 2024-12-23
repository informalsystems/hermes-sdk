/*!
   Trait definition for [`HasIbcPacketTypes`].
*/

use cgp::prelude::*;

use crate::traits::types::counterparty::CanUseCounterparty;

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

pub trait HasIncomingPacketType<Counterparty>:
    Sized
    + Async
    + CanUseCounterparty<
        Counterparty,
        Counterparty: HasOutgoingPacketType<Self, OutgoingPacket = Self::IncomingPacket>,
    >
{
    type IncomingPacket: Async;
}

impl<Chain, Counterparty> HasIncomingPacketType<Counterparty> for Chain
where
    Chain: Async,
    Counterparty: HasOutgoingPacketType<Chain>,
{
    type IncomingPacket = Counterparty::OutgoingPacket;
}
