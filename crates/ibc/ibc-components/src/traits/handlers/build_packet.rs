use alloc::vec::Vec;
use cgp::prelude::*;

use crate::traits::types::packet::data::HasPacketDataType;
use crate::traits::types::packet::entry::HasPacketEntryHeaderType;
use crate::traits::types::packet::nonce::HasPacketNonceType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::transaction_header::HasIbcTransactionHeaderType;

#[derive_component(PacketBuilderComponent, PacketBuilder<Chain>)]
#[async_trait]
pub trait CanBuildPacket<Counterparty, App>:
    HasPacketType<Counterparty>
    + HasIbcTransactionHeaderType<Counterparty>
    + HasPacketEntryHeaderType<Counterparty>
    + HasPacketNonceType<Counterparty>
    + HasPacketDataType<Counterparty, App>
    + HasErrorType
{
    async fn build_packet(
        &self,
        transaction_header: &Self::IbcTransactionHeader,
        nonce: Self::PacketNonce,
        entries: Vec<(Self::PacketEntryHeader, Self::PacketData)>,
    ) -> Result<Self::Packet, Self::Error>;
}
