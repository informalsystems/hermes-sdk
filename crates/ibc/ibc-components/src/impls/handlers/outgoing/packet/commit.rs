use core::marker::PhantomData;

use alloc::vec::Vec;

use crate::traits::commitment::path::send_packet::CanBuildSendPacketCommitmentPath;
use crate::traits::commitment::store::CanStoreCommitment;
use crate::traits::commitment::value::send_packet::CanBuildSendPacketCommitmentValue;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::handlers::outgoing::packet::PacketSender;
use crate::traits::types::payload::payload::HasPayloadType;
use crate::traits::types::transaction_header::HasIbcTransactionHeaderType;

pub struct CommitSendPacket<InHandler>(pub PhantomData<InHandler>);

impl<Chain, Counterparty, InHandler> PacketSender<Chain, Counterparty>
    for CommitSendPacket<InHandler>
where
    Chain: HasIbcTransactionHeaderType<Counterparty>
        + HasPayloadType<Counterparty>
        + HasPacketHeader<Counterparty>
        + CanBuildSendPacketCommitmentPath<Counterparty>
        + CanBuildSendPacketCommitmentValue<Counterparty>
        + CanStoreCommitment,
    InHandler: PacketSender<Chain, Counterparty>,
{
    async fn send_packet(
        chain: &Chain,
        transaction_header: &Chain::IbcTransactionHeader,
        payloads: Vec<Chain::Payload>,
    ) -> Result<Chain::Packet, Chain::Error> {
        let packet = InHandler::send_packet(chain, transaction_header, payloads).await?;

        let packet_header = Chain::packet_header(&packet);

        let commitment_path = Chain::build_send_packet_commitment_path(packet_header)?;

        let commitment_value = Chain::build_send_packet_commitment_value(&packet)?;

        chain
            .store_commitment(&commitment_path, &commitment_value)
            .await?;

        Ok(packet)
    }
}
