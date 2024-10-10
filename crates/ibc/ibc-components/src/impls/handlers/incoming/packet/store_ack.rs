use core::marker::PhantomData;

use hermes_chain_type_components::traits::types::commitment_proof::HasCommitmentProofType;

use crate::traits::commitment::path::ack_packet::CanBuildAckPacketCommitmentPath;
use crate::traits::commitment::store::CanStoreCommitment;
use crate::traits::commitment::value::ack_packet::CanBuildAckPacketCommitmentValue;
use crate::traits::commitment::value::send_packet::CanBuildRecvPacketCommitmentValue;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::handlers::incoming::packet::IncomingPacketHandler;
use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::packet::HasPacketType;

pub struct StorePacketAck<InHandler>(pub PhantomData<InHandler>);

impl<Chain, Counterparty, InHandler> IncomingPacketHandler<Chain, Counterparty>
    for StorePacketAck<InHandler>
where
    Chain: CanStoreCommitment
        + HasPacketAckType<Counterparty>
        + CanBuildAckPacketCommitmentPath<Counterparty>
        + CanBuildAckPacketCommitmentValue<Counterparty>
        + CanBuildRecvPacketCommitmentValue<Counterparty>,
    Counterparty: HasCommitmentProofType + HasPacketType<Chain> + HasPacketHeader<Chain>,
    InHandler: IncomingPacketHandler<Chain, Counterparty>,
{
    async fn handle_incoming_packet(
        chain: &Chain,
        packet: &Counterparty::Packet,
        send_proof: &Counterparty::CommitmentProof,
    ) -> Result<(), Chain::Error> {
        let ack = InHandler::handle_incoming_packet(chain, packet, send_proof).await?;

        let packet_header = Counterparty::packet_header(packet);

        let path = Chain::build_ack_packet_commitment_path(packet_header)?;

        let commitment_value = Chain::build_recv_packet_commitment_value(packet)?;

        chain.store_commitment(&path, &commitment_value).await?;

        Ok(ack)
    }
}
