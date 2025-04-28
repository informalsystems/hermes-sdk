/*!
   Trait definitions for [`HasSendPacketEvent`].
*/

use hermes_prelude::*;

/**
   Indicates that a chain context's
   [`Event`](crate::traits::HasEventType::Event)
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
