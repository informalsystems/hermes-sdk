use cgp::prelude::*;

use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::payload::payload::HasPayloadType;
use crate::traits::types::transaction_header::HasIbcTransactionHeaderType;

#[derive_component(PacketSenderComponent, PacketSender<Chain>)]
#[async_trait]
pub trait CanSendPacket<Counterparty>:
    HasIbcTransactionHeaderType<Counterparty>
    + HasPayloadType<Counterparty>
    + HasPacketType<Counterparty>
    + HasErrorType
{
    async fn send_packet(
        &self,
        transaction_header: &Self::IbcTransactionHeader,
        payloads: &[Self::Payload],
    ) -> Result<Self::Packet, Self::Error>;
}
