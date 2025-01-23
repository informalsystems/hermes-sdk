use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::packet::HasOutgoingPacketType;

use crate::traits::types::ibc_events::send_packet::HasSendPacketEvent;

#[cgp_component {
  provider: PacketFromSendPacketEventBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildPacketFromSendPacket<Counterparty>:
    Sized + HasSendPacketEvent<Counterparty> + HasOutgoingPacketType<Counterparty> + HasAsyncErrorType
{
    async fn build_packet_from_send_packet_event(
        &self,
        event: &Self::SendPacketEvent,
    ) -> Result<Self::OutgoingPacket, Self::Error>;
}
