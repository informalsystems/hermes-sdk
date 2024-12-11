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

pub trait HasIncomingPacketType<Counterparty>: Sized + Async {
    type IncomingPacket: Async;
}

impl<Chain, Counterparty> HasIncomingPacketType<Counterparty> for Chain
where
    Chain: Async,
    Counterparty: HasOutgoingPacketType<Chain>,
{
    type IncomingPacket = Counterparty::OutgoingPacket;
}

pub trait CanUseIncomingPacketType<Counterparty>:
    HasIncomingPacketType<Counterparty, IncomingPacket = Counterparty::OutgoingPacket>
where
    Counterparty: HasOutgoingPacketType<Self>,
{
}

impl<Chain, Counterparty> CanUseIncomingPacketType<Counterparty> for Chain
where
    Chain: Async,
    Counterparty: HasOutgoingPacketType<Chain>,
{
}
