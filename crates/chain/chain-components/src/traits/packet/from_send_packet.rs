use hermes_chain_type_components::traits::HasOutgoingPacketType;
use hermes_prelude::*;

use crate::traits::HasSendPacketEvent;

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
