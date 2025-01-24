/*!
   Trait definitions for [`HasSendPacketEvent`].
*/

use cgp::prelude::*;

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
pub trait HasSendPacketEvent<Counterparty> {
    type SendPacketEvent: Async;
}
