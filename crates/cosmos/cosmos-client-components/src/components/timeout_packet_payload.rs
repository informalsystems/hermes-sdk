use async_trait::async_trait;
use ibc_relayer::chain::handle::ChainHandle;
use hermes_relayer_components::chain::traits::components::timeout_unordered_packet_message_builder::TimeoutUnorderedPacketPayloadBuilder;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayload;
use ibc_relayer_types::core::ics04_channel::packet::{Packet, PacketMsgType};
use ibc_relayer_types::Height;

use crate::traits::chain_handle::HasBlockingChainHandle;
use crate::types::payloads::packet::CosmosTimeoutUnorderedPacketPayload;

pub struct BuildCosmosTimeoutPacketPayload;

#[async_trait]
impl<Chain, Counterparty> TimeoutUnorderedPacketPayloadBuilder<Chain, Counterparty>
    for BuildCosmosTimeoutPacketPayload
where
    Chain: HasTimeoutUnorderedPacketPayload<
            Counterparty,
            TimeoutUnorderedPacketPayload = CosmosTimeoutUnorderedPacketPayload,
        > + HasIbcPacketTypes<Counterparty, IncomingPacket = Packet>
        + HasClientStateType<Counterparty>
        + HasHeightType<Height = Height>
        + HasBlockingChainHandle,
{
    async fn build_timeout_unordered_packet_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Height,
        packet: &Packet,
    ) -> Result<CosmosTimeoutUnorderedPacketPayload, Chain::Error> {
        let height = *height;
        let packet = packet.clone();

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let proofs = chain_handle
                    .build_packet_proofs(
                        PacketMsgType::TimeoutUnordered,
                        &packet.destination_port,
                        &packet.destination_channel,
                        packet.sequence,
                        height,
                    )
                    .map_err(Chain::raise_error)?;

                Ok(CosmosTimeoutUnorderedPacketPayload {
                    update_height: proofs.height(),
                    proof_unreceived: proofs.object_proof().clone(),
                })
            })
            .await
    }
}
