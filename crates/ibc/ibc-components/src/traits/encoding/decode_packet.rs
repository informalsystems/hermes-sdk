use alloc::vec::Vec;
use cgp::prelude::*;

use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::raw_data::HasPacketRawDataType;

pub trait CanDecodePacket<Counterparty>:
    HasErrorType
    + HasPacketHeaderType<Counterparty>
    + HasPacketEntryHeaderType<Counterparty>
    + HasPacketRawDataType<Counterparty>
{
    fn decode_packet(
        &self,
        bytes: &[u8],
    ) -> Result<
        (
            Self::PacketHeader,
            Vec<(Self::PacketEntryHeader, Self::PacketRawData)>,
        ),
        Self::Error,
    >;
}
