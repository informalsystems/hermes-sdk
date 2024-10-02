use cgp::core::component::UseContext;
use cgp::prelude::*;

use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::payload::ack::HasPayloadAckType;
use crate::traits::types::payload::header::HasPayloadHeaderType;

#[derive_component(IncomingPacketEntryHandlerComponent, IncomingPacketEntryHandler<Chain>)]
#[async_trait]
pub trait CanHandleIncomingPacketEntry<Counterparty, App>:
    HasErrorType + HasPayloadAckType<Counterparty, App>
where
    Counterparty:
        HasPacketHeaderType<Self> + HasPayloadHeaderType<Self> + HasPacketDataType<Self, App>,
{
    async fn handle_incoming_packet_entry(
        &self,
        packet_header: &Counterparty::PacketHeader,
        entry_header: &Counterparty::PayloadHeader,
        entry_data: &Counterparty::PacketData,
    ) -> Result<Self::PayloadAck, Self::Error>;
}

impl<Chain, Counterparty, App> IncomingPacketEntryHandler<Chain, Counterparty, App> for UseContext
where
    Chain: CanHandleIncomingPacketEntry<Counterparty, App>,
    Counterparty:
        HasPacketHeaderType<Chain> + HasPayloadHeaderType<Chain> + HasPacketDataType<Chain, App>,
{
    async fn handle_incoming_packet_entry(
        chain: &Chain,
        packet_header: &Counterparty::PacketHeader,
        entry_header: &Counterparty::PayloadHeader,
        entry_data: &Counterparty::PacketData,
    ) -> Result<Chain::PayloadAck, Chain::Error> {
        chain
            .handle_incoming_packet_entry(packet_header, entry_header, entry_data)
            .await
    }
}
