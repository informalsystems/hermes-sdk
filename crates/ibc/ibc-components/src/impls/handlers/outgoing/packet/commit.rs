use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::commitment::path::send_packet::CanBuildSendPacketCommitmentPath;
use crate::traits::commitment::store::CanStoreCommitment;
use crate::traits::commitment::value::send_packet::CanBuildSendPacketCommitmentValue;
use crate::traits::fields::packet::header::channel_id::HasPacketChannelIds;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::fields::packet::packet::nonce::HasPacketNonce;
use crate::traits::handlers::outgoing::packet::PacketSender;
use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::payload::payload::HasPayloadType;
use crate::types::tags::commitment::send::SendPacket;

pub struct CommitSendPacket<InHandler>(pub PhantomData<InHandler>);

#[async_trait]
impl<Chain, Counterparty, InHandler> PacketSender<Chain, Counterparty>
    for CommitSendPacket<InHandler>
where
    Chain: HasPacketHeaderType<Counterparty>
        + HasPayloadType<Counterparty>
        + HasPacketHeader<Counterparty>
        + HasPacketChannelIds<Counterparty>
        + HasPacketNonce<Counterparty>
        + CanBuildSendPacketCommitmentPath<Counterparty>
        + CanBuildSendPacketCommitmentValue<Counterparty>
        + CanStoreCommitment<SendPacket>,
    Counterparty: HasChannelIdType<Chain>,
    InHandler: PacketSender<Chain, Counterparty>,
{
    async fn send_packet(
        chain: &mut Chain,
        packet_header: &Chain::PacketHeader,
        payloads: Vec<Chain::Payload>,
    ) -> Result<Chain::Packet, Chain::Error> {
        let packet = InHandler::send_packet(chain, packet_header, payloads).await?;

        let packet_header = Chain::packet_header(&packet);

        let src_channel_id = Chain::packet_src_channel_id(packet_header);
        let dst_channel_id = Chain::packet_dst_channel_id(packet_header);
        let nonce = Chain::packet_nonce(&packet);

        let commitment_path =
            Chain::build_send_packet_commitment_path(src_channel_id, dst_channel_id, nonce)?;

        let commitment_value = Chain::build_send_packet_commitment_value(&packet)?;

        chain
            .store_commitment(&commitment_path, &commitment_value)
            .await?;

        Ok(packet)
    }
}
