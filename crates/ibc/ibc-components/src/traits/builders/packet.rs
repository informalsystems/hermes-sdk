use alloc::vec::Vec;

use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::nonce::HasPacketNonceType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::payload::payload::HasPayloadType;

#[derive_component(PacketBuilderComponent, PacketBuilder<Chain>)]
#[async_trait]
pub trait CanBuildPacket<Counterparty>:
    HasPacketType<Counterparty>
    + HasPacketHeaderType<Counterparty>
    + HasPacketNonceType<Counterparty>
    + HasPayloadType<Counterparty>
    + HasErrorType
{
    async fn build_packet(
        &self,
        packet_header: &Self::PacketHeader,
        nonce: Self::PacketNonce,
        payloads: Vec<Self::Payload>,
    ) -> Result<Self::Packet, Self::Error>;
}
