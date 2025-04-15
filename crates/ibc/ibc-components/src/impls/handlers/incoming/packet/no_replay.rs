use core::fmt::Debug;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::HasChannelIdType;

use crate::traits::fields::packet::header::channel_id::HasPacketChannelIds;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::fields::packet::packet::nonce::HasPacketNonce;
use crate::traits::handlers::incoming::packet::{
    IncomingPacketHandler, IncomingPacketHandlerComponent,
};
use crate::traits::queries::recv_packet_commitment::CanQueryHasPacketReceived;
use crate::traits::types::commitment::proof::HasCommitmentProofType;
use crate::traits::types::packet::nonce::HasPacketNonceType;
use crate::traits::types::packet::packet::HasPacketType;
use crate::types::tags::commitment::send::SendPacket;

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

#[cgp_provider(IncomingPacketHandlerComponent)]
#[async_trait]
impl<Chain, Counterparty, InHandler> IncomingPacketHandler<Chain, Counterparty>
    for DisallowDoubleReceive<InHandler>
where
    Chain: CanQueryHasPacketReceived<Counterparty>
        + for<'a> CanRaiseAsyncError<DoublePacketReceive<'a, Chain, Counterparty>>,
    Counterparty: HasCommitmentProofType<SendPacket>
        + HasPacketHeader<Chain>
        + HasPacketNonce<Chain>
        + HasChannelIdType<Chain>
        + HasPacketChannelIds<Chain>,
    InHandler: IncomingPacketHandler<Chain, Counterparty>,
{
    async fn handle_incoming_packet(
        chain: &mut Chain,
        packet: &Counterparty::Packet,
        send_proof: &Counterparty::CommitmentProof,
    ) -> Result<(), Chain::Error> {
        let packet_header = Counterparty::packet_header(packet);
        let nonce = Counterparty::packet_nonce(packet);
        let src_channel_id = Counterparty::packet_src_channel_id(packet_header);
        let dst_channel_id = Counterparty::packet_dst_channel_id(packet_header);

        let received = chain
            .query_has_packet_received(src_channel_id, dst_channel_id, nonce)
            .await?;

        if received {
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

impl<Chain, Counterparty> Debug for DoublePacketReceive<'_, Chain, Counterparty>
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
