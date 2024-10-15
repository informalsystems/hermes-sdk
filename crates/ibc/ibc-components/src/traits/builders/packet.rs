use alloc::vec::Vec;
use cgp::prelude::*;

use crate::traits::types::packet::nonce::HasPacketNonceType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::payload::payload::HasPayloadType;
use crate::traits::types::transaction_header::HasIbcTransactionHeaderType;

#[derive_component(PacketBuilderComponent, PacketBuilder<Chain>)]
#[async_trait]
pub trait CanBuildPacket<Counterparty>:
    HasPacketType<Counterparty>
    + HasIbcTransactionHeaderType<Counterparty>
    + HasPacketNonceType<Counterparty>
    + HasPayloadType<Counterparty>
    + HasErrorType
{
    async fn build_packet(
        &self,
        transaction_header: &Self::IbcTransactionHeader,
        nonce: Self::PacketNonce,
        payloads: Vec<Self::Payload>,
    ) -> Result<Self::Packet, Self::Error>;
}
