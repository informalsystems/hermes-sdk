use cgp_core::prelude::*;
use cgp_core::HasErrorType;
use hermes_relayer_components::chain::traits::components::receive_packet_message_builder::ReceivePacketMessageBuilder;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayloadType;
use ibc_relayer_types::core::ics04_channel::packet::Packet;

use crate::traits::message::{CosmosMessage, ToCosmosMessage};
use crate::types::messages::packet::receive::CosmosReceivePacketMessage;
use crate::types::payloads::packet::CosmosReceivePacketPayload;

pub struct BuildCosmosReceivePacketMessage;

#[async_trait]
impl<Chain, Counterparty> ReceivePacketMessageBuilder<Chain, Counterparty>
    for BuildCosmosReceivePacketMessage
where
    Chain: HasMessageType<Message = CosmosMessage>
        + HasIbcPacketTypes<Counterparty, IncomingPacket = Packet>
        + HasErrorType,
    Counterparty:
        HasReceivePacketPayloadType<Chain, ReceivePacketPayload = CosmosReceivePacketPayload>,
{
    async fn build_receive_packet_message(
        _chain: &Chain,
        packet: &Packet,
        payload: CosmosReceivePacketPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        let message = CosmosReceivePacketMessage {
            packet: packet.clone(),
            update_height: payload.update_height,
            proof_commitment: payload.proof_commitment,
        };

        Ok(message.to_cosmos_message())
    }
}
