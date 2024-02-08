use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::payload_builders::receive_packet::ReceivePacketPayloadBuilder;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayloadType;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_types::core::ics04_channel::packet::{Packet, PacketMsgType};
use ibc_relayer_types::Height;

use crate::traits::chain_handle::HasBlockingChainHandle;
use crate::types::payloads::packet::CosmosReceivePacketPayload;

pub struct BuildCosmosReceivePacketPayload;

#[async_trait]
impl<Chain, Counterparty> ReceivePacketPayloadBuilder<Chain, Counterparty>
    for BuildCosmosReceivePacketPayload
where
    Chain: HasReceivePacketPayloadType<Counterparty, ReceivePacketPayload = CosmosReceivePacketPayload>
        + HasIbcPacketTypes<Counterparty, OutgoingPacket = Packet>
        + HasClientStateType<Counterparty>
        + HasHeightType<Height = Height>
        + HasBlockingChainHandle,
{
    async fn build_receive_packet_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Height,
        packet: &Packet,
    ) -> Result<CosmosReceivePacketPayload, Chain::Error> {
        let height = *height;
        let packet = packet.clone();

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let proofs = chain_handle
                    .build_packet_proofs(
                        PacketMsgType::Recv,
                        &packet.source_port,
                        &packet.source_channel,
                        packet.sequence,
                        height,
                    )
                    .map_err(Chain::raise_error)?;

                Ok(CosmosReceivePacketPayload {
                    update_height: proofs.height(),
                    proof_commitment: proofs.object_proof().clone(),
                })
            })
            .await
    }
}
