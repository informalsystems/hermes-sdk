use core::marker::PhantomData;

use alloc::vec::Vec;
use cgp::prelude::CanRaiseError;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_chain_type_components::traits::types::ibc::consensus_state::HasConsensusStateType;

use crate::traits::commitment::path::send_packet::CanBuildSendPacketCommitmentPath;
use crate::traits::commitment::value::send_packet::CanBuildSendPacketCommitmentValue;
use crate::traits::commitment::verify::value::CanVerifyValueCommitment;
use crate::traits::fields::commitment::proof_height::HasCommitmentProofHeight;
use crate::traits::fields::packet::header::client::HasPacketClients;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::handlers::incoming::packet::IncomingPacketHandler;
use crate::traits::queries::consensus_state::CanQueryConsensusState;
use crate::traits::types::packet::ack::HasPacketAckType;

pub struct VerifySendPacketCommitmentProof<InHandler>(pub PhantomData<InHandler>);

impl<Chain, Counterparty, App, InHandler> IncomingPacketHandler<Chain, App, Counterparty>
    for VerifySendPacketCommitmentProof<InHandler>
where
    Chain: HasPacketAckType<App, Counterparty>
        + CanQueryConsensusState<Counterparty>
        + CanRaiseError<Counterparty::Error>,
    Counterparty: HasHeightType
        + HasCommitmentProofHeight
        + HasPacketHeader<Chain>
        + HasPacketClients<Chain>
        + HasConsensusStateType<Chain>
        + CanVerifyValueCommitment<Chain>
        + CanBuildSendPacketCommitmentPath<Chain>
        + CanBuildSendPacketCommitmentValue<Chain>,
    InHandler: IncomingPacketHandler<Chain, App, Counterparty>,
{
    async fn handle_incoming_packet(
        chain: &Chain,
        packet: &Counterparty::Packet,
        send_proof: &Counterparty::CommitmentProof,
    ) -> Result<Vec<Chain::PacketAck>, Chain::Error> {
        let header = Counterparty::packet_header(packet);
        let client_id = Counterparty::packet_dst_client_id(header);
        let proof_height = Counterparty::commitment_proof_height(send_proof);

        let consensus_state = chain.query_consensus_state(client_id, proof_height).await?;

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
