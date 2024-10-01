use cgp::prelude::*;

use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::entry_ack::HasPacketEntryAckType;
use crate::traits::types::packet::header::HasPacketHeaderType;

#[derive_component(PacketAckEntriesHandlerComponent, PacketAckEntriesHandler<Chain>)]
#[async_trait]
pub trait CanHandlePacketAckEntries<Counterparty, App>:
    HasErrorType + HasPacketHeaderType<Counterparty> + HasPacketEntryHeaderType<Counterparty>
where
    Counterparty: HasPacketEntryAckType<Self, App>,
{
    async fn handle_packet_ack_entries(
        &self,
        header: &Self::PacketHeader,
        ack_entries: &[(Self::PacketEntryHeader, Counterparty::PacketEntryAck)],
    ) -> Result<(), Self::Error>;
}
