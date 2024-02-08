use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::payload_builders::ack_packet::AckPacketPayloadBuilder;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc_events::write_ack::HasWriteAckEvent;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::chain::traits::types::packets::ack::HasAckPacketPayloadType;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_types::core::ics04_channel::events::WriteAcknowledgement;
use ibc_relayer_types::core::ics04_channel::packet::{Packet, PacketMsgType};
use ibc_relayer_types::Height;

use crate::traits::chain_handle::HasBlockingChainHandle;
use crate::types::payloads::packet::CosmosAckPacketPayload;

pub struct BuildCosmosAckPacketPayload;

#[async_trait]
impl<Chain, Counterparty> AckPacketPayloadBuilder<Chain, Counterparty>
    for BuildCosmosAckPacketPayload
where
    Chain: HasAckPacketPayloadType<Counterparty, AckPacketPayload = CosmosAckPacketPayload>
        + HasWriteAckEvent<Counterparty, WriteAckEvent = WriteAcknowledgement>
        + HasIbcPacketTypes<Counterparty, IncomingPacket = Packet>
        + HasClientStateType<Counterparty>
        + HasHeightType<Height = Height>
        + HasBlockingChainHandle,
{
    async fn build_ack_packet_payload(
        chain: &Chain,
        _client_state: &Chain::ClientState,
        height: &Height,
        packet: &Packet,
        ack: &WriteAcknowledgement,
    ) -> Result<CosmosAckPacketPayload, Chain::Error> {
        let height = *height;
        let packet = packet.clone();
        let ack = ack.clone();

        chain
            .with_blocking_chain_handle(move |chain_handle| {
                let proofs = chain_handle
                    .build_packet_proofs(
                        PacketMsgType::Ack,
                        &packet.destination_port,
                        &packet.destination_channel,
                        packet.sequence,
                        height,
                    )
                    .map_err(Chain::raise_error)?;

                let ack = ack.ack;

                Ok(CosmosAckPacketPayload {
                    ack,
                    update_height: proofs.height(),
                    proof_acked: proofs.object_proof().clone(),
                })
            })
            .await
    }
}
