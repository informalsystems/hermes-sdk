use core::fmt::Debug;
use core::marker::PhantomData;

use cgp::prelude::CanRaiseError;
use hermes_chain_type_components::traits::types::commitment_proof::HasCommitmentProofType;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::fields::packet::header::channel::HasPacketChannels;
use crate::traits::fields::packet::header::nonce::HasPacketNonce;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::handlers::incoming::packet::IncomingPacketHandler;
use crate::traits::queries::ack_packet_commitment::CanQueryAckPacketCommitment;
use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::nonce::HasPacketNonceType;
use crate::traits::types::packet::packet::HasPacketType;

pub struct DisallowDoubleReceive<InHandler>(pub PhantomData<InHandler>);

pub struct DoublePacketReceive<'a, Chain, Counterparty>
where
    Chain: HasChannelIdType<Counterparty>,
    Counterparty: HasChannelIdType<Chain> + HasPacketNonceType<Chain> + HasPacketType<Chain>,
{
    pub src_channel_id: &'a Counterparty::ChannelId,
    pub dst_channel_id: &'a Chain::ChannelId,
    pub nonce: &'a Counterparty::PacketNonce,
    pub packet: &'a Counterparty::Packet,
}

impl<Chain, Counterparty, InHandler> IncomingPacketHandler<Chain, Counterparty>
    for DisallowDoubleReceive<InHandler>
where
    Chain: HasPacketAckType<Counterparty>
        + CanQueryAckPacketCommitment<Counterparty>
        + for<'a> CanRaiseError<DoublePacketReceive<'a, Chain, Counterparty>>,
    Counterparty: HasCommitmentProofType
        + HasPacketHeader<Chain>
        + HasPacketNonce<Chain>
        + HasChannelIdType<Chain>
        + HasPacketChannels<Chain>,
    InHandler: IncomingPacketHandler<Chain, Counterparty>,
{
    async fn handle_incoming_packet(
        chain: &Chain,
        packet: &Counterparty::Packet,
        send_proof: &Counterparty::CommitmentProof,
    ) -> Result<Chain::PacketAck, Chain::Error> {
        let packet_header = Counterparty::packet_header(packet);
        let nonce = Counterparty::packet_nonce(packet_header);
        let src_channel_id = Counterparty::packet_src_channel_id(packet_header);
        let dst_channel_id = Counterparty::packet_dst_channel_id(packet_header);

        let m_ack = chain
            .query_ack_packet_commitment(src_channel_id, dst_channel_id, nonce)
            .await?;

        if m_ack.is_some() {
            Err(Chain::raise_error(DoublePacketReceive {
                src_channel_id,
                dst_channel_id,
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
    Chain: HasChannelIdType<Counterparty>,
    Counterparty: HasChannelIdType<Chain> + HasPacketNonceType<Chain> + HasPacketType<Chain>,
    Chain::ChannelId: Debug,
    Counterparty::ChannelId: Debug,
    Counterparty::PacketNonce: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "chain has already received incoming packet from {:?} to {:?} with nonce {:?}",
            self.src_channel_id, self.dst_channel_id, self.nonce,
        )?;

        Ok(())
    }
}
