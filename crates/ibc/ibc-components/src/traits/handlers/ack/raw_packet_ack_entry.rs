use cgp::prelude::*;

use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::raw_ack::HasPacketRawAckType;

#[derive_component(RawPacketAckEntryHandlerComponent, RawPacketAckEntryHandler<Chain>)]
#[async_trait]
pub trait CanHandleRawPacketAckEntry<Counterparty>:
    HasErrorType + HasPacketHeaderType<Counterparty> + HasPacketEntryHeaderType<Counterparty>
where
    Counterparty: HasPacketRawAckType<Self>,
{
    async fn handle_raw_packet_ack_entry(
        &self,
        packet_header: &Self::PacketHeader,
        entry_header: &Self::PacketEntryHeader,
        entry_ack: &Counterparty::PacketRawAck,
    ) -> Result<(), Self::Error>;
}
