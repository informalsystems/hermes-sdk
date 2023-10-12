use alloc::sync::Arc;
use async_trait::async_trait;
use cgp_core::HasErrorType;
use ibc_relayer_components::chain::traits::components::timeout_unordered_packet_message_builder::TimeoutUnorderedPacketMessageBuilder;
use ibc_relayer_components::chain::traits::types::message::HasMessageType;
use ibc_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use ibc_relayer_components::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayload;
use ibc_relayer_types::core::ics04_channel::packet::Packet;

use crate::traits::message::{CosmosMessage, ToCosmosMessage};
use crate::types::messages::packet::timeout::CosmosTimeoutPacketMessage;
use crate::types::payloads::packet::CosmosTimeoutUnorderedPacketPayload;

pub struct BuildCosmosTimeoutPacketMessage;

#[async_trait]
impl<Chain, Counterparty> TimeoutUnorderedPacketMessageBuilder<Chain, Counterparty>
    for BuildCosmosTimeoutPacketMessage
where
    Chain: HasMessageType<Message = Arc<dyn CosmosMessage>>
        + HasErrorType
        + HasIbcPacketTypes<Counterparty, OutgoingPacket = Packet>,
    Counterparty: HasTimeoutUnorderedPacketPayload<
        Chain,
        TimeoutUnorderedPacketPayload = CosmosTimeoutUnorderedPacketPayload,
    >,
{
    async fn build_timeout_unordered_packet_message(
        _chain: &Chain,
        packet: &Packet,
        payload: Counterparty::TimeoutUnorderedPacketPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Chain::Error> {
        let message = CosmosTimeoutPacketMessage {
            next_sequence_recv: packet.sequence,
            packet: packet.clone(),
            update_height: payload.update_height,
            proof_unreceived: payload.proof_unreceived,
        };

        Ok(message.to_cosmos_message())
    }
}
