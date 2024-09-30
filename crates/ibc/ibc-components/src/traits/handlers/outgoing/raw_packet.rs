use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::payload::HasPacketPayloadHeaderType;
use crate::traits::types::packet::raw_data::HasPacketRawDataType;
use crate::traits::types::packet::timeout::HasPacketTimeoutType;

#[derive_component(OutgoingRawPacketHandlerComponent, OutgoingRawPacketHandler<Chain>)]
#[async_trait]
pub trait CanBuildOutgoingPacketHeader<Counterparty>:
    HasErrorType
    + HasClientIdType<Counterparty>
    + HasPacketHeaderType<Counterparty>
    + HasPacketPayloadHeaderType<Counterparty>
    + HasPacketRawDataType<Counterparty>
where
    Counterparty: HasClientIdType<Self> + HasPacketTimeoutType<Self>,
{
    async fn build_outgoing_packet_header(
        &self,
        source_client_id: &Self::ClientId,
        destination_client_id: &Counterparty::ClientId,
        timeout: &Counterparty::PacketTimeout,
        payloads: &[(Self::PacketPayloadHeader, Self::PacketRawData)],
    ) -> Result<Self::PacketHeader, Self::Error>;
}
