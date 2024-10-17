use core::marker::PhantomData;

use hermes_chain_type_components::traits::types::commitment_proof::HasCommitmentProofType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::commitment::path::receive_packet::CanBuildReceivePacketCommitmentPath;
use crate::traits::commitment::store::CanStoreCommitment;
use crate::traits::commitment::value::receive_packet::CanBuildReceivePacketCommitmentValue;
use crate::traits::fields::packet::header::channel_id::HasPacketChannelIds;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::fields::packet::packet::nonce::HasPacketNonce;
use crate::traits::handlers::incoming::packet::IncomingPacketHandler;

pub struct StoreReceivePacket<InHandler>(pub PhantomData<InHandler>);

impl<Chain, Counterparty, InHandler> IncomingPacketHandler<Chain, Counterparty>
    for StoreReceivePacket<InHandler>
where
    Chain: CanStoreCommitment
        + HasChannelIdType<Counterparty>
        + CanBuildReceivePacketCommitmentPath<Counterparty>
        + CanBuildReceivePacketCommitmentValue<Counterparty>,
    Counterparty: HasCommitmentProofType
        + HasPacketHeader<Chain>
        + HasPacketChannelIds<Chain>
        + HasPacketNonce<Chain>,
    InHandler: IncomingPacketHandler<Chain, Counterparty>,
{
    async fn handle_incoming_packet(
        chain: &Chain,
        packet: &Counterparty::Packet,
        send_proof: &Counterparty::CommitmentProof,
    ) -> Result<(), Chain::Error> {
        InHandler::handle_incoming_packet(chain, packet, send_proof).await?;

        let packet_header = Counterparty::packet_header(packet);

        let src_channel_id = Counterparty::packet_src_channel_id(packet_header);
        let dst_channel_id = Counterparty::packet_dst_channel_id(packet_header);
        let nonce = Counterparty::packet_nonce(packet);

        let path =
            Chain::build_receive_packet_commitment_path(src_channel_id, dst_channel_id, nonce)?;

        let commitment_value = Chain::build_receive_packet_commitment_value(packet)?;

        chain.store_commitment(&path, &commitment_value).await?;

        Ok(())
    }
}
