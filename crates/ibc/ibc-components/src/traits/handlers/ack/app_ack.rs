use cgp::prelude::*;

use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::header::HasPacketHeaderType;

#[derive_component(PacketAckHandlerComponent, PacketPayloadHandler<Chain>)]
#[async_trait]
pub trait CanHandlePacketAck<App, Counterparty>:
    HasErrorType
    + HasPacketHeaderType<Counterparty>
    + HasPacketEntryHeaderType<Counterparty>
    + HasPacketDataType<App, Counterparty>
where
    Counterparty: HasPacketAckType<App, Self>,
{
    async fn handle_packet_ack(
        &self,
        packet_header: &Self::PacketHeader,
        entry_header: &Self::PacketEntryHeader,
        entry_ack: &Counterparty::PacketAck,
    ) -> Result<(), Self::Error>;
}
