use alloc::vec::Vec;
use cgp::prelude::HasErrorType;
use hermes_chain_type_components::traits::types::commitment_proof::HasCommitmentProofType;

use crate::traits::fields::packet::packet::entries::HasPacketEntries;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::handlers::incoming::packet::IncomingPacketHandler;
use crate::traits::handlers::incoming::packet_entry::CanHandleIncomingPacketEntry;
use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::packet::HasPacketType;

pub struct HandleIncomingPacketEntries;

impl<Chain, Counterparty, App> IncomingPacketHandler<Chain, Counterparty, App>
    for HandleIncomingPacketEntries
where
    Chain: HasErrorType
        + HasPacketAckType<Counterparty, App>
        + CanHandleIncomingPacketEntry<Counterparty, App>,
    Counterparty: HasCommitmentProofType
        + HasPacketType<Chain>
        + HasPacketHeader<Chain>
        + HasPacketEntries<Chain, App>,
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
