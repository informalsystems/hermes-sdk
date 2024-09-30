use alloc::vec::Vec;
use cgp::prelude::HasErrorType;

use crate::traits::fields::packet::packet::entries::HasPacketEntries;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::handlers::incoming::app_packet::CanHandleIncomingPacketEntry;
use crate::traits::handlers::incoming::packet::IncomingPacketHandler;
use crate::traits::types::commitment::proof::HasCommitmentProofType;
use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::types::any_app::AnyApp;

pub struct HandleIncomingPacketEntries;

impl<Chain, Counterparty> IncomingPacketHandler<Chain, Counterparty> for HandleIncomingPacketEntries
where
    Chain: HasErrorType
        + HasPacketAckType<AnyApp, Counterparty>
        + CanHandleIncomingPacketEntry<AnyApp, Counterparty>,
    Counterparty: HasCommitmentProofType
        + HasPacketType<Chain>
        + HasPacketHeader<Chain>
        + HasPacketEntries<Chain>,
{
    async fn handle_incoming_packet(
        chain: &Chain,
        packet: &Counterparty::Packet,
        _send_proof: &Counterparty::CommitmentProof,
    ) -> Result<Vec<Chain::PacketAck>, Chain::Error> {
        let packet_header = Counterparty::packet_header(packet);
        let packet_entries = Counterparty::packet_entries(packet);

        let mut acks = Vec::new();

        for (entry_header, entry_data) in packet_entries {
            let ack = chain
                .handle_incoming_packet_entry(packet_header, entry_header, entry_data)
                .await?;
            acks.push(ack);
        }

        Ok(acks)
    }
}
