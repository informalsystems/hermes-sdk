use core::marker::PhantomData;

use cgp::prelude::*;

use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::fields::packet::packet::payloads::HasPacketPayloads;
use crate::traits::fields::payload::data::HasPayloadData;
use crate::traits::fields::payload::header::HasPayloadHeader;
use crate::traits::handlers::incoming::packet::IncomingPacketHandler;
use crate::traits::handlers::incoming::payload::CanHandleIncomingPayload;
use crate::traits::types::commitment::proof::HasCommitmentProofType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::types::tags::commitment::send::SendPacket;

pub struct HandleIncomingPacketPayloads<App>(pub PhantomData<App>);

#[async_trait]
impl<Chain, Counterparty, App> IncomingPacketHandler<Chain, Counterparty>
    for HandleIncomingPacketPayloads<App>
where
    Chain: HasAsyncErrorType + CanHandleIncomingPayload<Counterparty, App>,
    Counterparty: HasCommitmentProofType<SendPacket>
        + HasPacketType<Chain>
        + HasPacketHeader<Chain>
        + HasPacketPayloads<Chain>
        + HasPayloadHeader<Chain>
        + HasPayloadData<Chain, App>,
{
    async fn handle_incoming_packet(
        chain: &mut Chain,
        packet: &Counterparty::Packet,
        _send_proof: &Counterparty::CommitmentProof,
    ) -> Result<(), Chain::Error> {
        let packet_header = Counterparty::packet_header(packet);
        let payloads = Counterparty::packet_payloads(packet);

        for payload in payloads {
            let payload_header = Counterparty::payload_header(payload);
            let payload_data = Counterparty::payload_data(payload);

            chain
                .handle_incoming_payload(packet_header, payload_header, payload_data)
                .await?;
        }

        Ok(())
    }
}
