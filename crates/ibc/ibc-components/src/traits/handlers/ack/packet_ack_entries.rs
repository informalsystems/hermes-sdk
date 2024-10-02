use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::payload::ack::HasPayloadAckType;
use crate::traits::types::payload::header::HasPayloadHeaderType;

#[derive_component(PacketAckEntriesHandlerComponent, PacketAckEntriesHandler<Chain>)]
#[async_trait]
pub trait CanHandlePacketAckEntries<Counterparty, App>:
    HasErrorType + HasPacketHeaderType<Counterparty> + HasPayloadHeaderType<Counterparty>
where
    Counterparty: HasPayloadAckType<Self, App>,
{
    async fn handle_packet_ack_entries(
        &self,
        header: &Self::PacketHeader,
        ack_entries: &[(Self::PayloadHeader, Counterparty::PayloadAck)],
    ) -> Result<(), Self::Error>;
}
