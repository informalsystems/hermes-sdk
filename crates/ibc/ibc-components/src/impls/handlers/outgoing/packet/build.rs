use alloc::vec::Vec;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::builders::packet::CanBuildPacket;
use crate::traits::fields::transaction::channel_id::HasIbcTransactionChannelIds;
use crate::traits::handlers::outgoing::packet::PacketSender;
use crate::traits::nonce::CanAllocatePacketNonce;
use crate::traits::types::packet::packet::HasPacketType;
use crate::traits::types::payload::payload::HasPayloadType;

pub struct AllocateNonceAndBuildPacket;

impl<Chain, Counterparty> PacketSender<Chain, Counterparty> for AllocateNonceAndBuildPacket
where
    Chain: HasIbcTransactionChannelIds<Counterparty>
        + HasPayloadType<Counterparty>
        + HasPacketType<Counterparty>
        + CanBuildPacket<Counterparty>
        + CanAllocatePacketNonce<Counterparty>,
    Counterparty: HasChannelIdType<Chain>,
{
    async fn send_packet(
        chain: &Chain,
        transaction_header: &Chain::IbcTransactionHeader,
        payloads: Vec<Chain::Payload>,
    ) -> Result<Chain::Packet, Chain::Error> {
        let src_channel_id = Chain::transaction_src_channel_id(transaction_header);
        let dst_channel_id = Chain::transaction_dst_channel_id(transaction_header);

        let nonce = chain
            .allocate_packet_nonce(src_channel_id, dst_channel_id)
            .await?;

        let packet = chain
            .build_packet(transaction_header, nonce, payloads)
            .await?;

        Ok(packet)
    }
}
