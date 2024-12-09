use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::ibc::consensus_state::HasConsensusStateType;

use crate::traits::commitment::path::send_packet::CanBuildSendPacketCommitmentPath;
use crate::traits::commitment::value::send_packet::CanBuildSendPacketCommitmentValue;
use crate::traits::commitment::verify::CanVerifyCommitment;
use crate::traits::fields::commitment::proof_height::HasCommitmentProofHeight;
use crate::traits::fields::packet::header::channel_id::HasPacketChannelIds;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::fields::packet::packet::nonce::HasPacketNonce;
use crate::traits::handlers::incoming::packet::IncomingPacketHandler;
use crate::traits::queries::client_id::CanQueryClientIdFromChannelId;
use crate::traits::queries::consensus_state::CanQueryConsensusState;
use crate::types::tags::commitment::send::SendPacket;

pub struct VerifySendPacketCommitmentProof<InHandler>(pub PhantomData<InHandler>);

#[async_trait]
impl<Chain, Counterparty, InHandler> IncomingPacketHandler<Chain, Counterparty>
    for VerifySendPacketCommitmentProof<InHandler>
where
    Chain: CanQueryConsensusState<Counterparty>
        + CanRaiseError<Counterparty::Error>
        + CanQueryClientIdFromChannelId<Counterparty>,
    Counterparty: HasHeightType
        + HasCommitmentProofHeight<SendPacket>
        + HasPacketHeader<Chain>
        + HasPacketChannelIds<Chain>
        + HasPacketNonce<Chain>
        + HasConsensusStateType<Chain>
        + CanVerifyCommitment<Chain, SendPacket>
        + CanBuildSendPacketCommitmentPath<Chain>
        + CanBuildSendPacketCommitmentValue<Chain>,
    InHandler: IncomingPacketHandler<Chain, Counterparty>,
{
    async fn handle_incoming_packet(
        chain: &mut Chain,
        packet: &Counterparty::Packet,
        send_proof: &Counterparty::CommitmentProof,
    ) -> Result<(), Chain::Error> {
        let packet_header = Counterparty::packet_header(packet);

        let src_channel_id = Counterparty::packet_src_channel_id(packet_header);
        let dst_channel_id = Counterparty::packet_dst_channel_id(packet_header);
        let nonce = Counterparty::packet_nonce(packet);

        let proof_height = Counterparty::commitment_proof_height(send_proof);

        let client_id = chain
            .query_client_id_from_channel_id(dst_channel_id)
            .await?;

        let consensus_state = chain
            .query_consensus_state(&client_id, proof_height)
            .await?;

        let commitment_path =
            Counterparty::build_send_packet_commitment_path(src_channel_id, dst_channel_id, nonce)
                .map_err(Chain::raise_error)?;

        let commitment_value =
            Counterparty::build_send_packet_commitment_value(packet).map_err(Chain::raise_error)?;

        Counterparty::verify_commitment(
            &consensus_state,
            &commitment_path,
            &commitment_value,
            send_proof,
        )
        .map_err(Chain::raise_error)?;

        InHandler::handle_incoming_packet(chain, packet, send_proof).await
    }
}
