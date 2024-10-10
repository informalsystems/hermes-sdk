use alloc::vec::Vec;
use cgp::prelude::*;

use crate::traits::types::packet::nonce::HasPacketNonceType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::payload::data::HasPayloadDataType;
use crate::traits::types::payload::header::HasPayloadHeaderType;
use crate::traits::types::transaction_header::HasIbcTransactionHeaderType;

#[derive_component(PacketBuilderComponent, PacketBuilder<Chain>)]
#[async_trait]
pub trait CanBuildPacket<Counterparty, App>:
    HasPacketType<Counterparty>
    + HasIbcTransactionHeaderType<Counterparty>
    + HasPayloadHeaderType<Counterparty>
    + HasPacketNonceType<Counterparty>
    + HasPayloadDataType<Counterparty, App>
    + HasErrorType
{
    async fn build_packet(
        &self,
        transaction_header: &Self::IbcTransactionHeader,
        nonce: Self::PacketNonce,
        payloads: Vec<(Self::PayloadHeader, Self::PayloadData)>,
    ) -> Result<Self::Packet, Self::Error>;
}
