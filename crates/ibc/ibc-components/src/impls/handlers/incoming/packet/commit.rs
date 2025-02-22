use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::components::chain::re_exports::IncomingPacketHandlerComponent;
use crate::traits::commitment::path::receive_packet::CanBuildReceivePacketCommitmentPath;
use crate::traits::commitment::store::CanStoreCommitment;
use crate::traits::commitment::value::receive_packet::CanBuildReceivePacketCommitmentValue;
use crate::traits::fields::packet::header::channel_id::HasPacketChannelIds;
use crate::traits::fields::packet::packet::header::HasPacketHeader;
use crate::traits::fields::packet::packet::nonce::HasPacketNonce;
use crate::traits::handlers::incoming::packet::IncomingPacketHandler;
use crate::traits::types::commitment::proof::HasCommitmentProofType;
use crate::types::tags::commitment::receive::ReceivePacket;
use crate::types::tags::commitment::send::SendPacket;

pub struct CommitReceivePacket<InHandler>(pub PhantomData<InHandler>);

#[cgp_provider(IncomingPacketHandlerComponent)]
#[async_trait]
impl<Chain, Counterparty, InHandler> IncomingPacketHandler<Chain, Counterparty>
    for CommitReceivePacket<InHandler>
where
    Chain: CanStoreCommitment<ReceivePacket>
        + HasChannelIdType<Counterparty>
        + CanBuildReceivePacketCommitmentPath<Counterparty>
        + CanBuildReceivePacketCommitmentValue<Counterparty>,
    Counterparty: HasCommitmentProofType<SendPacket>
        + HasPacketHeader<Chain>
        + HasPacketChannelIds<Chain>
        + HasPacketNonce<Chain>,
    InHandler: IncomingPacketHandler<Chain, Counterparty>,
{
    async fn handle_incoming_packet(
        chain: &mut Chain,
        packet: &Counterparty::Packet,
        send_proof: &Counterparty::CommitmentProof,
    ) -> Result<(), Chain::Error> {
        InHandler::handle_incoming_packet(chain, packet, send_proof).await?;

        let packet_header = Counterparty::packet_header(packet);

        let src_channel_id = Counterparty::packet_src_channel_id(packet_header);
        let dst_channel_id = Counterparty::packet_dst_channel_id(packet_header);
        let nonce = Counterparty::packet_nonce(packet);

        let path =
            Chain::build_receive_packet_commitment_path(src_channel_id, dst_channel_id, nonce)?;

        let commitment_value = Chain::build_receive_packet_commitment_value(packet)?;

        chain.store_commitment(&path, &commitment_value).await?;

        Ok(())
    }
}
