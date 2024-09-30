use cgp::prelude::*;

use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::raw_ack::HasPacketRawAckType;

#[derive_component(RawPacketAckHandlerComponent, RawPacketAckHandler<Chain>)]
#[async_trait]
pub trait CanHandleRawPacketAck<Counterparty>:
    HasErrorType + HasPacketHeaderType<Counterparty> + HasPacketEntryHeaderType<Counterparty>
where
    Counterparty: HasPacketRawAckType<Self>,
{
    async fn handle_raw_packet_ack(
        &self,
        header: &Self::PacketHeader,
        ack_entries: &[(Self::PacketEntryHeader, Counterparty::PacketRawAck)],
    ) -> Result<(), Self::Error>;
}
