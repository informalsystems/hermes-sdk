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
