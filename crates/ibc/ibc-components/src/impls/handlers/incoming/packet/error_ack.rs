use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::commitment_proof::HasCommitmentProofType;

use crate::traits::builders::ack_error::ErrorAsAckWrapper;
use crate::traits::handlers::incoming::packet::IncomingPacketHandler;
use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::packet::HasPacketType;

pub struct WrapHandlerErrorAsAck<ErrorHandler, InHandler>(
    pub PhantomData<(ErrorHandler, InHandler)>,
);

impl<Chain, Counterparty, ErrorHandler, InHandler> IncomingPacketHandler<Chain, Counterparty>
    for WrapHandlerErrorAsAck<ErrorHandler, InHandler>
where
    Chain: HasErrorType + HasPacketAckType<Counterparty>,
    Counterparty: HasCommitmentProofType + HasPacketType<Chain>,
    InHandler: IncomingPacketHandler<Chain, Counterparty>,
    ErrorHandler: ErrorAsAckWrapper<Chain, Counterparty>,
{
    async fn handle_incoming_packet(
        chain: &Chain,
        packet: &Counterparty::Packet,
        send_proof: &Counterparty::CommitmentProof,
    ) -> Result<Chain::PacketAck, Chain::Error> {
        let res = InHandler::handle_incoming_packet(chain, packet, send_proof).await;

        match res {
            Ok(ack) => Ok(ack),
            Err(e) => ErrorHandler::try_wrap_error_as_ack(e),
        }
    }
}
