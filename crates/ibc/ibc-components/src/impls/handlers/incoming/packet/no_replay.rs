use core::fmt::Debug;
use core::marker::PhantomData;

use alloc::vec::Vec;
use cgp::prelude::{CanRaiseError, HasErrorType};
use hermes_chain_type_components::traits::types::commitment_proof::HasCommitmentProofType;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;

use crate::traits::fields::packet::header::client::HasPacketClients;
use crate::traits::fields::packet::header::nonce::HasPacketNonce;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::handlers::incoming::packet::IncomingPacketHandler;
use crate::traits::queries::packet_ack::CanQueryPacketAck;
use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::nonce::HasPacketNonceType;
use crate::traits::types::packet::packet::HasPacketType;

pub struct DisallowDoubleReceive<InHandler>(pub PhantomData<InHandler>);

pub struct DoublePacketReceive<'a, Chain, Counterparty>
where
    Chain: HasClientIdType<Counterparty>,
    Counterparty: HasClientIdType<Chain> + HasPacketNonceType<Chain> + HasPacketType<Chain>,
{
    pub src_client_id: &'a Counterparty::ClientId,
    pub dst_client_id: &'a Chain::ClientId,
    pub nonce: &'a Counterparty::PacketNonce,
    pub packet: &'a Counterparty::Packet,
}

impl<Chain, Counterparty, App, InHandler> IncomingPacketHandler<Chain, Counterparty, App>
    for DisallowDoubleReceive<InHandler>
where
    Chain: HasErrorType
        + HasPacketAckType<Counterparty, App>
        + CanQueryPacketAck<Counterparty, App>
        + for<'a> CanRaiseError<DoublePacketReceive<'a, Chain, Counterparty>>,
    Counterparty: HasCommitmentProofType
        + HasPacketHeader<Chain>
        + HasPacketNonce<Chain>
        + HasClientIdType<Chain>
        + HasPacketClients<Chain>,
    InHandler: IncomingPacketHandler<Chain, Counterparty, App>,
{
    async fn handle_incoming_packet(
        chain: &Chain,
        packet: &Counterparty::Packet,
        send_proof: &Counterparty::CommitmentProof,
    ) -> Result<Vec<Chain::PacketAck>, Chain::Error> {
        let packet_header = Counterparty::packet_header(packet);
        let nonce = Counterparty::packet_nonce(packet_header);
        let src_client_id = Counterparty::packet_src_client_id(packet_header);
        let dst_client_id = Counterparty::packet_dst_client_id(packet_header);

        let m_ack = chain
            .query_packet_ack(src_client_id, dst_client_id, nonce)
            .await?;

        if m_ack.is_some() {
            Err(Chain::raise_error(DoublePacketReceive {
                src_client_id,
                dst_client_id,
                nonce,
                packet,
            }))
        } else {
            InHandler::handle_incoming_packet(chain, packet, send_proof).await
        }
    }
}

impl<'a, Chain, Counterparty> Debug for DoublePacketReceive<'a, Chain, Counterparty>
where
    Chain: HasClientIdType<Counterparty>,
    Counterparty: HasClientIdType<Chain> + HasPacketNonceType<Chain> + HasPacketType<Chain>,
    Chain::ClientId: Debug,
    Counterparty::ClientId: Debug,
    Counterparty::PacketNonce: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "chain has already received incoming packet from {:?} to {:?} with nonce {:?}",
            self.src_client_id, self.dst_client_id, self.nonce,
        )?;

        Ok(())
    }
}
