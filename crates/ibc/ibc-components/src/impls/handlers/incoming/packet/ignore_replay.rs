use core::marker::PhantomData;

use hermes_chain_type_components::traits::types::commitment_proof::HasCommitmentProofType;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;

use crate::traits::fields::packet::header::client::HasPacketClients;
use crate::traits::fields::packet::header::nonce::HasPacketNonce;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::handlers::incoming::packet::IncomingPacketHandler;
use crate::traits::queries::ack_packet_commitment::CanQueryAckPacketCommitment;
use crate::traits::types::packet::ack::HasPacketAckType;

pub struct IgnoreDoubleReceive<InHandler>(pub PhantomData<InHandler>);

impl<Chain, Counterparty, InHandler> IncomingPacketHandler<Chain, Counterparty>
    for IgnoreDoubleReceive<InHandler>
where
    Chain: HasPacketAckType<Counterparty> + CanQueryAckPacketCommitment<Counterparty>,
    Counterparty: HasCommitmentProofType
        + HasPacketHeader<Chain>
        + HasPacketNonce<Chain>
        + HasClientIdType<Chain>
        + HasPacketClients<Chain>,
    InHandler: IncomingPacketHandler<Chain, Counterparty>,
{
    async fn handle_incoming_packet(
        chain: &Chain,
        packet: &Counterparty::Packet,
        send_proof: &Counterparty::CommitmentProof,
    ) -> Result<Chain::PacketAck, Chain::Error> {
        let packet_header = Counterparty::packet_header(packet);
        let nonce = Counterparty::packet_nonce(packet_header);
        let src_client_id = Counterparty::packet_src_client_id(packet_header);
        let dst_client_id = Counterparty::packet_dst_client_id(packet_header);

        let m_ack = chain
            .query_ack_packet_commitment(src_client_id, dst_client_id, nonce)
            .await?;

        match m_ack {
            Some(ack) => Ok(ack),
            None => InHandler::handle_incoming_packet(chain, packet, send_proof).await,
        }
    }
}