use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::fields::packet::header::channel_id::HasPacketChannelIds;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::fields::packet::packet::nonce::HasPacketNonce;
use crate::traits::handlers::incoming::packet::{
    IncomingPacketHandler, IncomingPacketHandlerComponent,
};
use crate::traits::queries::recv_packet_commitment::CanQueryHasPacketReceived;
use crate::traits::types::commitment::proof::HasCommitmentProofType;
use crate::types::tags::commitment::send::SendPacket;

pub struct IgnoreDoubleReceive<InHandler>(pub PhantomData<InHandler>);

#[cgp_provider(IncomingPacketHandlerComponent)]
#[async_trait]
impl<Chain, Counterparty, InHandler> IncomingPacketHandler<Chain, Counterparty>
    for IgnoreDoubleReceive<InHandler>
where
    Chain: CanQueryHasPacketReceived<Counterparty>,
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
            Ok(())
        } else {
            InHandler::handle_incoming_packet(chain, packet, send_proof).await
        }
    }
}
