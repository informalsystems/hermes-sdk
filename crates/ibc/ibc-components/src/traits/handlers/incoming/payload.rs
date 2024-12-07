use cgp::core::component::{UseContext, UseDelegate};
use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::payload::data::HasPayloadDataType;
use crate::traits::types::payload::header::HasPayloadHeaderType;

#[cgp_component {
  name: IncomingPayloadHandlerComponent,
  provider: IncomingPayloadHandler,
  context: Chain,
}]
#[async_trait]
pub trait CanHandleIncomingPayload<Counterparty, App>: Sized + Async + HasErrorType
where
    Counterparty:
        HasPacketHeaderType<Self> + HasPayloadHeaderType<Self> + HasPayloadDataType<Self, App>,
{
    async fn handle_incoming_payload(
        &mut self,
        packet_header: &Counterparty::PacketHeader,
        payload_header: &Counterparty::PayloadHeader,
        payload_data: &Counterparty::PayloadData,
    ) -> Result<(), Self::Error>;
}

#[async_trait]
impl<Chain, Counterparty, App, Components> IncomingPayloadHandler<Chain, Counterparty, App>
    for UseDelegate<Components>
where
    Chain: Async + HasErrorType,
    Counterparty:
        HasPacketHeaderType<Chain> + HasPayloadHeaderType<Chain> + HasPayloadDataType<Chain, App>,
    Components: DelegateComponent<App>,
    Components::Delegate: IncomingPayloadHandler<Chain, Counterparty, App>,
{
    async fn handle_incoming_payload(
        chain: &mut Chain,
        packet_header: &Counterparty::PacketHeader,
        payload_header: &Counterparty::PayloadHeader,
        payload_data: &Counterparty::PayloadData,
    ) -> Result<(), Chain::Error> {
        Components::Delegate::handle_incoming_payload(
            chain,
            packet_header,
            payload_header,
            payload_data,
        )
        .await
    }
}

#[async_trait]
impl<Chain, Counterparty, App> IncomingPayloadHandler<Chain, Counterparty, App> for UseContext
where
    Chain: CanHandleIncomingPayload<Counterparty, App>,
    Counterparty:
        HasPacketHeaderType<Chain> + HasPayloadHeaderType<Chain> + HasPayloadDataType<Chain, App>,
{
    async fn handle_incoming_payload(
        chain: &mut Chain,
        packet_header: &Counterparty::PacketHeader,
        payload_header: &Counterparty::PayloadHeader,
        payload_data: &Counterparty::PayloadData,
    ) -> Result<(), Chain::Error> {
        chain
            .handle_incoming_payload(packet_header, payload_header, payload_data)
            .await
    }
}
