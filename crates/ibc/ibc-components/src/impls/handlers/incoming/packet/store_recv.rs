use core::marker::PhantomData;

use hermes_chain_type_components::traits::types::commitment_proof::HasCommitmentProofType;

use crate::traits::commitment::path::recv_packet::CanBuildRecvPacketCommitmentPath;
use crate::traits::commitment::store::CanStoreCommitment;
use crate::traits::commitment::value::send_packet::CanBuildRecvPacketCommitmentValue;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::handlers::incoming::packet::IncomingPacketHandler;
use crate::traits::types::packet::packet::HasPacketType;

pub struct StoreRecvPacket<InHandler>(pub PhantomData<InHandler>);

impl<Chain, Counterparty, InHandler> IncomingPacketHandler<Chain, Counterparty>
    for StoreRecvPacket<InHandler>
where
    Chain: CanStoreCommitment
        + CanBuildRecvPacketCommitmentPath<Counterparty>
        + CanBuildRecvPacketCommitmentValue<Counterparty>,
    Counterparty: HasCommitmentProofType + HasPacketType<Chain> + HasPacketHeader<Chain>,
    InHandler: IncomingPacketHandler<Chain, Counterparty>,
{
    async fn handle_incoming_packet(
        chain: &Chain,
        packet: &Counterparty::Packet,
        send_proof: &Counterparty::CommitmentProof,
    ) -> Result<(), Chain::Error> {
        InHandler::handle_incoming_packet(chain, packet, send_proof).await?;

        let packet_header = Counterparty::packet_header(packet);

        let path = Chain::build_recv_packet_commitment_path(packet_header)?;

        let commitment_value = Chain::build_recv_packet_commitment_value(packet)?;

        chain.store_commitment(&path, &commitment_value).await?;

        Ok(())
    }
}
