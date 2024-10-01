use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;

use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::timeout::HasPacketTimeoutType;

#[derive_component(PacketSenderComponent, PacketSender<Chain>)]
#[async_trait]
pub trait CanSendPacket<Counterparty, App>:
    HasErrorType
    + HasClientIdType<Counterparty>
    + HasPacketHeaderType<Counterparty>
    + HasPacketEntryHeaderType<Counterparty>
    + HasPacketDataType<Counterparty, App>
where
    Counterparty: HasClientIdType<Self> + HasPacketTimeoutType<Self>,
{
    async fn send_packet(
        &self,
        source_client_id: &Self::ClientId,
        destination_client_id: &Counterparty::ClientId,
        timeout: &Counterparty::PacketTimeout,
        payloads: &[(Self::PacketEntryHeader, Self::PacketData)],
    ) -> Result<Self::PacketHeader, Self::Error>;
}
