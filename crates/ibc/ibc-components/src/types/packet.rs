use core::marker::PhantomData;

use alloc::vec::Vec;
use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::packet::ProvidePacketType;
use crate::traits::types::payload::payload::HasPayloadType;

#[derive(HasField)]
pub struct IbcPacket<Chain, Counterparty>
where
    Chain: HasPacketHeaderType<Counterparty> + HasPayloadType<Counterparty>,
{
    pub header: Chain::PacketHeader,
    pub payloads: Vec<Chain::Payload>,
}

pub struct UseIbcPacket<App>(pub PhantomData<App>);

impl<Chain, Counterparty, App> ProvidePacketType<Chain, Counterparty> for UseIbcPacket<App>
where
    Chain: HasPacketHeaderType<Counterparty> + HasPayloadType<Counterparty>,
    Counterparty: Async,
    App: Async,
{
    type Packet = IbcPacket<Chain, Counterparty>;
}

impl<Chain, Counterparty> Clone for IbcPacket<Chain, Counterparty>
where
    Chain: HasPacketHeaderType<Counterparty, PacketHeader: Clone>
        + HasPayloadType<Counterparty, Payload: Clone>,
{
    fn clone(&self) -> Self {
        Self {
            header: self.header.clone(),
            payloads: self.payloads.clone(),
        }
    }
}
