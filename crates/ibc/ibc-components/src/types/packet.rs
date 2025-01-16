use alloc::vec::Vec;

use cgp::prelude::*;

use crate::traits::builders::packet::PacketBuilder;
use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::nonce::HasPacketNonceType;
use crate::traits::types::packet::packet::{HasPacketType, ProvidePacketType};
use crate::traits::types::payload::payload::HasPayloadType;

#[derive(HasField)]
pub struct IbcPacket<Chain, Counterparty>
where
    Chain: HasPacketHeaderType<Counterparty>
        + HasPacketNonceType<Counterparty>
        + HasPayloadType<Counterparty>,
{
    pub header: Chain::PacketHeader,
    pub nonce: Chain::PacketNonce,
    pub payloads: Vec<Chain::Payload>,
}

pub struct UseIbcPacket;

impl<Chain, Counterparty> ProvidePacketType<Chain, Counterparty> for UseIbcPacket
where
    Chain: HasPacketHeaderType<Counterparty>
        + HasPacketNonceType<Counterparty>
        + HasPayloadType<Counterparty>,
    Counterparty: Async,
{
    type Packet = IbcPacket<Chain, Counterparty>;
}

impl<Chain, Counterparty> PacketBuilder<Chain, Counterparty> for UseIbcPacket
where
    Chain: HasPacketHeaderType<Counterparty, PacketHeader: Clone>
        + HasPacketNonceType<Counterparty>
        + HasPayloadType<Counterparty>
        + HasPacketType<Counterparty, Packet = IbcPacket<Chain, Counterparty>>
        + HasAsyncErrorType,
    Counterparty: Async,
{
    fn build_packet(
        packet_header: &Chain::PacketHeader,
        nonce: Chain::PacketNonce,
        payloads: Vec<Chain::Payload>,
    ) -> Result<IbcPacket<Chain, Counterparty>, Chain::Error> {
        Ok(IbcPacket {
            header: packet_header.clone(),
            nonce,
            payloads,
        })
    }
}

impl<Chain, Counterparty> Clone for IbcPacket<Chain, Counterparty>
where
    Chain: HasPacketHeaderType<Counterparty, PacketHeader: Clone>
        + HasPacketNonceType<Counterparty, PacketNonce: Clone>
        + HasPayloadType<Counterparty, Payload: Clone>,
{
    fn clone(&self) -> Self {
        Self {
            header: self.header.clone(),
            nonce: self.nonce.clone(),
            payloads: self.payloads.clone(),
        }
    }
}

impl<Chain, Counterparty> PartialEq for IbcPacket<Chain, Counterparty>
where
    Chain: HasPacketHeaderType<Counterparty, PacketHeader: Eq>
        + HasPacketNonceType<Counterparty, PacketNonce: Eq>
        + HasPayloadType<Counterparty, Payload: Eq>,
{
    fn eq(&self, other: &Self) -> bool {
        self.header == other.header && self.nonce == other.nonce && self.payloads == other.payloads
    }
}

impl<Chain, Counterparty> Eq for IbcPacket<Chain, Counterparty> where
    Chain: HasPacketHeaderType<Counterparty, PacketHeader: Eq>
        + HasPacketNonceType<Counterparty, PacketNonce: Eq>
        + HasPayloadType<Counterparty, Payload: Eq>
{
}
