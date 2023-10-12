use alloc::sync::Arc;

use async_trait::async_trait;
use cgp_core::HasErrorType;
use ibc_relayer_components::chain::traits::components::ack_packet_message_builder::AckPacketMessageBuilder;
use ibc_relayer_components::chain::traits::types::message::HasMessageType;
use ibc_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use ibc_relayer_components::chain::traits::types::packets::ack::HasAckPacketPayload;
use ibc_relayer_types::core::ics04_channel::packet::Packet;

use crate::traits::message::{CosmosMessage, ToCosmosMessage};
use crate::types::messages::packet::ack::CosmosAckPacketMessage;
use crate::types::payloads::packet::CosmosAckPacketPayload;

pub struct BuildCosmosAckPacketMessage;

#[async_trait]
impl<Chain, Counterparty> AckPacketMessageBuilder<Chain, Counterparty>
    for BuildCosmosAckPacketMessage
where
    Chain: HasMessageType<Message = Arc<dyn CosmosMessage>>
        + HasErrorType
        + HasIbcPacketTypes<Counterparty, OutgoingPacket = Packet>,
    Counterparty: HasAckPacketPayload<Chain, AckPacketPayload = CosmosAckPacketPayload>,
{
    async fn build_ack_packet_message(
        _chain: &Chain,
        packet: &Packet,
        payload: CosmosAckPacketPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Chain::Error> {
        let message = CosmosAckPacketMessage {
            packet: packet.clone(),
            acknowledgement: payload.ack,
            update_height: payload.update_height,
            proof_acked: payload.proof_acked,
        };

        Ok(message.to_cosmos_message())
    }
}
