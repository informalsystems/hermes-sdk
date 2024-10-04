use alloc::vec::Vec;
use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
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
