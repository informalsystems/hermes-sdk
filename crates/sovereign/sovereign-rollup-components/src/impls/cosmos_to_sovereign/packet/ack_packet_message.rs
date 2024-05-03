use cgp_core::prelude::*;
use cgp_core::HasErrorType;

use hermes_cosmos_chain_components::traits::message::ToCosmosMessage;
use hermes_cosmos_chain_components::types::messages::packet::ack::CosmosAckPacketMessage;
use hermes_cosmos_chain_components::types::payloads::packet::CosmosAckPacketPayload;
use hermes_relayer_components::chain::traits::message_builders::ack_packet::AckPacketMessageBuilder;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::chain::traits::types::packets::ack::HasAckPacketPayloadType;
use ibc_relayer_types::core::ics04_channel::packet::Packet;

use crate::types::message::SovereignMessage;

pub struct BuildAckPacketMessageOnSovereign;

#[async_trait]
impl<Rollup, Counterparty> AckPacketMessageBuilder<Rollup, Counterparty>
    for BuildAckPacketMessageOnSovereign
where
    Rollup: HasMessageType<Message = SovereignMessage>
        + HasIbcPacketTypes<Counterparty, OutgoingPacket = Packet>
        + HasErrorType,
    Counterparty: HasAckPacketPayloadType<Rollup, AckPacketPayload = CosmosAckPacketPayload>,
{
    async fn build_ack_packet_message(
        _rollup: &Rollup,
        packet: &Packet,
        payload: CosmosAckPacketPayload,
    ) -> Result<SovereignMessage, Rollup::Error> {
        let message = CosmosAckPacketMessage {
            packet: packet.clone(),
            update_height: payload.update_height,
            proof_acked: payload.proof_acked,
            acknowledgement: payload.ack,
        };

        let cosmos_message = message.to_cosmos_message();
        let sovereign_message: SovereignMessage = cosmos_message.into();

        Ok(sovereign_message)
    }
}
