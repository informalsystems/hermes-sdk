use core::marker::PhantomData;

use alloc::vec::Vec;

use crate::traits::commitment::path::ack_packet::CanBuildAckPacketCommitmentPath;
use crate::traits::commitment::store::CanStoreCommitment;
use crate::traits::commitment::value::ack_packet::CanBuildAckPacketCommitmentValue;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::handlers::incoming::packet::IncomingPacketHandler;
use crate::traits::types::commitment::proof::HasCommitmentProofType;
use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::types::any_app::AnyApp;

pub struct StorePacketAck<InHandler>(pub PhantomData<InHandler>);

impl<Chain, Counterparty, InHandler> IncomingPacketHandler<Chain, Counterparty>
    for StorePacketAck<InHandler>
where
    Chain: CanStoreCommitment
        + HasPacketAckType<AnyApp, Counterparty>
        + CanBuildAckPacketCommitmentPath<Counterparty>
        + CanBuildAckPacketCommitmentValue<Counterparty>,
    Counterparty: HasCommitmentProofType + HasPacketType<Chain> + HasPacketHeader<Chain>,
    InHandler: IncomingPacketHandler<Chain, Counterparty>,
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
