use core::marker::PhantomData;

use alloc::vec::Vec;
use cgp::prelude::HasErrorType;
use hermes_chain_type_components::traits::types::commitment_proof::HasCommitmentProofType;

use crate::traits::builders::ack::CanBuildPacketAckFromEntries;
use crate::traits::fields::packet::packet::entries::HasPacketEntries;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::handlers::incoming::packet::IncomingPacketHandler;
use crate::traits::handlers::incoming::payload::CanHandleIncomingPayload;
use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::packet::HasPacketType;

pub struct HandleIncomingPacketEntries<App>(pub PhantomData<App>);

impl<Chain, Counterparty, App> IncomingPacketHandler<Chain, Counterparty>
    for HandleIncomingPacketEntries<App>
where
    Chain: HasErrorType
        + HasPacketAckType<Counterparty>
        + CanHandleIncomingPayload<Counterparty, App>
        + CanBuildPacketAckFromEntries<Counterparty, App>,
    Counterparty: HasCommitmentProofType
        + HasPacketType<Chain>
        + HasPacketHeader<Chain>
        + HasPacketEntries<Chain, App>,
{
    async fn handle_incoming_packet(
        chain: &Chain,
        packet: &Counterparty::Packet,
        _send_proof: &Counterparty::CommitmentProof,
    ) -> Result<Chain::PacketAck, Chain::Error> {
        let packet_header = Counterparty::packet_header(packet);
        let packet_entries = Counterparty::packet_entries(packet);

        let mut acks = Vec::new();

        for (payload_header, payload_data) in packet_entries {
            let ack = chain
                .handle_incoming_payload(packet_header, payload_header, payload_data)
                .await?;

            acks.push(ack);
        }

        let ack = Chain::build_packet_ack_from_entries(acks);

        Ok(ack)
    }
}
