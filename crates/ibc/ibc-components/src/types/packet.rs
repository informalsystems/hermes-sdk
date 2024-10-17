use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::nonce::HasPacketNonceType;
use crate::traits::types::packet::packet::ProvidePacketType;
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

pub struct UseIbcPacket<App>(pub PhantomData<App>);

impl<Chain, Counterparty, App> ProvidePacketType<Chain, Counterparty> for UseIbcPacket<App>
where
    Chain: HasPacketHeaderType<Counterparty>
        + HasPacketNonceType<Counterparty>
        + HasPayloadType<Counterparty>,
    Counterparty: Async,
    App: Async,
{
    type Packet = IbcPacket<Chain, Counterparty>;
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
