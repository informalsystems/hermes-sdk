use core::marker::PhantomData;

use alloc::vec::Vec;
use hermes_chain_type_components::traits::types::commitment_proof::HasCommitmentProofType;

use crate::traits::commitment::path::ack_packet::CanBuildAckPacketCommitmentPath;
use crate::traits::commitment::store::CanStoreCommitment;
use crate::traits::commitment::value::ack_packet::CanBuildAckPacketCommitmentValue;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::handlers::incoming::packet::IncomingPacketHandler;
use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::packet::HasPacketType;

pub struct StorePacketAck<InHandler>(pub PhantomData<InHandler>);

impl<Chain, Counterparty, App, InHandler> IncomingPacketHandler<Chain, Counterparty, App>
    for StorePacketAck<InHandler>
where
    Chain: CanStoreCommitment
        + HasPacketAckType<Counterparty, App>
        + CanBuildAckPacketCommitmentPath<Counterparty>
        + CanBuildAckPacketCommitmentValue<Counterparty, App>,
    Counterparty: HasCommitmentProofType + HasPacketType<Chain> + HasPacketHeader<Chain>,
    InHandler: IncomingPacketHandler<Chain, Counterparty, App>,
{
    async fn handle_incoming_packet(
        chain: &Chain,
        packet: &Counterparty::Packet,
        send_proof: &Counterparty::CommitmentProof,
    ) -> Result<Vec<Chain::PacketAck>, Chain::Error> {
        let acks = InHandler::handle_incoming_packet(chain, packet, send_proof).await?;

        let packet_header = Counterparty::packet_header(packet);

        let path = Chain::build_ack_packet_commitment_path(packet_header)?;

        let value = Chain::build_ack_packet_commitment_value(packet_header, &acks)?;

        chain.store_commitment(&path, &value).await?;

        Ok(acks)
    }
}
