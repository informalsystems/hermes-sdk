/*!
   Trait definitions for [`HasSendPacketEvent`].
*/

use cgp::prelude::*;

use crate::chain::traits::types::event::HasEventType;
use crate::chain::traits::types::packet::HasIbcPacketTypes;

/**
   Indicates that a chain context's
   [`Event`](crate::chain::traits::types::event::HasEventType::Event)
   type contains a [`SendPacketEvent`](Self::SendPacketEvent) variant.
*/
#[derive_component(SendPacketEventComponent, ProvideSendPacketEvent<Chain>)]
pub trait HasSendPacketEvent<Counterparty>: HasIbcPacketTypes<Counterparty> + HasEventType {
    type SendPacketEvent: Async;

    fn try_extract_send_packet_event(event: &Self::Event) -> Option<Self::SendPacketEvent>;

    fn extract_packet_from_send_packet_event(event: &Self::SendPacketEvent)
        -> Self::OutgoingPacket;
}
