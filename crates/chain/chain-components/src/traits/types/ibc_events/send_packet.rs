/*!
   Trait definitions for [`HasSendPacketEvent`].
*/

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::packet::HasOutgoingPacketType;

/**
   Indicates that a chain context's
   [`Event`](crate::traits::types::event::HasEventType::Event)
   type contains a [`SendPacketEvent`](Self::SendPacketEvent) variant.
*/
#[cgp_component {
  name: SendPacketEventComponent,
  provider: ProvideSendPacketEvent,
  context: Chain,
}]
pub trait HasSendPacketEvent<Counterparty>: HasOutgoingPacketType<Counterparty> {
    type SendPacketEvent: Async;

    fn extract_packet_from_send_packet_event(event: &Self::SendPacketEvent)
        -> Self::OutgoingPacket;
}
