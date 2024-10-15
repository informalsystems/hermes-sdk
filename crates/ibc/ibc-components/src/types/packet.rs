use core::marker::PhantomData;

use alloc::vec::Vec;
use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::packet::ProvidePacketType;
use crate::traits::types::payload::data::HasPayloadDataType;
use crate::traits::types::payload::header::HasPayloadHeaderType;

#[derive(HasField)]
pub struct IbcPacket<Chain, Counterparty, App>
where
    Chain: HasPacketHeaderType<Counterparty>
        + HasPayloadHeaderType<Counterparty>
        + HasPayloadDataType<Counterparty, App>,
{
    pub header: Chain::PacketHeader,
    pub payloads: Vec<(Chain::PayloadHeader, Chain::PayloadData)>,
}

pub struct UseIbcPacket<App>(pub PhantomData<App>);

impl<Chain, Counterparty, App> ProvidePacketType<Chain, Counterparty> for UseIbcPacket<App>
where
    Chain: HasPacketHeaderType<Counterparty>
        + HasPayloadHeaderType<Counterparty>
        + HasPayloadDataType<Counterparty, App>,
    Counterparty: Async,
    App: Async,
{
    type Packet = IbcPacket<Chain, Counterparty, App>;
}

impl<Chain, Counterparty, App> Clone for IbcPacket<Chain, Counterparty, App>
where
    Chain: HasPacketHeaderType<Counterparty, PacketHeader: Clone>
        + HasPayloadHeaderType<Counterparty, PayloadHeader: Clone>
        + HasPayloadDataType<Counterparty, App, PayloadData: Clone>,
{
    fn clone(&self) -> Self {
        Self {
            header: self.header.clone(),
            payloads: self.payloads.clone(),
        }
    }
}
