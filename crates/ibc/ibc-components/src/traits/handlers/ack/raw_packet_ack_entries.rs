use cgp::prelude::*;

use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::types::any_app::AnyApp;

#[derive_component(RawPacketAckEntriesHandlerComponent, RawPacketAckEntriesHandler<Chain>)]
#[async_trait]
pub trait CanHandleRawPacketAckEntries<Counterparty>:
    HasErrorType + HasPacketHeaderType<Counterparty> + HasPacketEntryHeaderType<Counterparty>
where
    Counterparty: HasPacketAckType<AnyApp, Self>,
{
    async fn handle_raw_packet_ack_entries(
        &self,
        header: &Self::PacketHeader,
        ack_entries: &[(Self::PacketEntryHeader, Counterparty::PacketAck)],
    ) -> Result<(), Self::Error>;
}
