use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::ibc::consensus_state::HasConsensusStateType;

use crate::traits::commitment::path::send_packet::CanBuildSendPacketCommitmentPath;
use crate::traits::commitment::value::send_packet::CanBuildSendPacketCommitmentValue;
use crate::traits::commitment::verify::value::CanVerifyValueCommitment;
use crate::traits::fields::commitment::proof_height::HasCommitmentProofHeight;
use crate::traits::fields::packet::header::channel_id::HasPacketChannelIds;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::handlers::incoming::packet::IncomingPacketHandler;
use crate::traits::queries::client_id::CanQueryClientIdFromChannelId;
use crate::traits::queries::consensus_state::CanQueryConsensusState;

pub struct VerifySendPacketCommitmentProof<InHandler>(pub PhantomData<InHandler>);

impl<Chain, Counterparty, InHandler> IncomingPacketHandler<Chain, Counterparty>
    for VerifySendPacketCommitmentProof<InHandler>
where
    Chain: CanQueryConsensusState<Counterparty>
        + CanRaiseError<Counterparty::Error>
        + CanQueryClientIdFromChannelId<Counterparty>,
    Counterparty: HasHeightType
        + HasCommitmentProofHeight
        + HasPacketHeader<Chain>
        + HasPacketChannelIds<Chain>
        + HasConsensusStateType<Chain>
        + CanVerifyValueCommitment<Chain>
        + CanBuildSendPacketCommitmentPath<Chain>
        + CanBuildSendPacketCommitmentValue<Chain>,
    InHandler: IncomingPacketHandler<Chain, Counterparty>,
{
    async fn handle_incoming_packet(
        chain: &Chain,
        packet: &Counterparty::Packet,
        send_proof: &Counterparty::CommitmentProof,
    ) -> Result<(), Chain::Error> {
        let header = Counterparty::packet_header(packet);
        let channel_id = Counterparty::packet_dst_channel_id(header);
        let proof_height = Counterparty::commitment_proof_height(send_proof);

        let client_id = chain.query_client_id_from_channel_id(channel_id).await?;

        let consensus_state = chain
            .query_consensus_state(&client_id, proof_height)
            .await?;

        let commitment_path =
            Counterparty::build_send_packet_commitment_path(header).map_err(Chain::raise_error)?;

        let commitment_value =
            Counterparty::build_send_packet_commitment_value(packet).map_err(Chain::raise_error)?;

        Counterparty::verify_value_commitment(
            &consensus_state,
            &commitment_path,
            &commitment_value,
            send_proof,
        )
        .map_err(Chain::raise_error)?;

        InHandler::handle_incoming_packet(chain, packet, send_proof).await
    }
}
