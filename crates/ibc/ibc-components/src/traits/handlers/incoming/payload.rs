use cgp::core::component::UseContext;
use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::payload::data::HasPayloadDataType;
use crate::traits::types::payload::header::HasPayloadHeaderType;

#[derive_component(IncomingPayloadHandlerComponent, IncomingPayloadHandler<Chain>)]
#[async_trait]
pub trait CanHandleIncomingPayload<Counterparty, App>: Async + HasErrorType
where
    Counterparty:
        HasPacketHeaderType<Self> + HasPayloadHeaderType<Self> + HasPayloadDataType<Self, App>,
{
    async fn handle_incoming_payload(
        &self,
        packet_header: &Counterparty::PacketHeader,
        payload_header: &Counterparty::PayloadHeader,
        payload_data: &Counterparty::PayloadData,
    ) -> Result<(), Self::Error>;
}

impl<Chain, Counterparty, App> IncomingPayloadHandler<Chain, Counterparty, App> for UseContext
where
    Chain: CanHandleIncomingPayload<Counterparty, App>,
    Counterparty:
        HasPacketHeaderType<Chain> + HasPayloadHeaderType<Chain> + HasPayloadDataType<Chain, App>,
{
    async fn handle_incoming_payload(
        chain: &Chain,
        packet_header: &Counterparty::PacketHeader,
        payload_header: &Counterparty::PayloadHeader,
        payload_data: &Counterparty::PayloadData,
    ) -> Result<(), Chain::Error> {
        chain
            .handle_incoming_payload(packet_header, payload_header, payload_data)
            .await
    }
}
