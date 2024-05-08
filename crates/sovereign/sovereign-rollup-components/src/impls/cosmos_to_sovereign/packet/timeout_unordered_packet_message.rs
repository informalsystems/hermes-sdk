use cgp_core::HasErrorType;

use hermes_cosmos_chain_components::traits::message::ToCosmosMessage;
use hermes_cosmos_chain_components::types::messages::packet::timeout::CosmosTimeoutPacketMessage;
use hermes_cosmos_chain_components::types::payloads::packet::CosmosTimeoutUnorderedPacketPayload;
use hermes_relayer_components::chain::traits::message_builders::timeout_unordered_packet::TimeoutUnorderedPacketMessageBuilder;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::chain::traits::types::packets::timeout::HasTimeoutUnorderedPacketPayloadType;
use ibc_relayer_types::core::ics04_channel::packet::Packet;

use crate::types::message::SovereignMessage;

pub struct BuildTimeoutPacketMessageOnSovereign;

impl<Rollup, Counterparty> TimeoutUnorderedPacketMessageBuilder<Rollup, Counterparty>
    for BuildTimeoutPacketMessageOnSovereign
where
    Rollup: HasMessageType<Message = SovereignMessage>
        + HasErrorType
        + HasIbcPacketTypes<Counterparty, OutgoingPacket = Packet>,
    Counterparty: HasTimeoutUnorderedPacketPayloadType<
        Rollup,
        TimeoutUnorderedPacketPayload = CosmosTimeoutUnorderedPacketPayload,
    >,
{
    async fn build_timeout_unordered_packet_message(
        _rollup: &Rollup,
        packet: &Packet,
        payload: CosmosTimeoutUnorderedPacketPayload,
    ) -> Result<SovereignMessage, Rollup::Error> {
        let msg = CosmosTimeoutPacketMessage {
            packet: packet.clone(),
            next_sequence_recv: packet.sequence,
            update_height: payload.update_height,
            proof_unreceived: payload.proof_unreceived,
        };

        let cosmos_msg = msg.to_cosmos_message();
        let sovereign_msg: SovereignMessage = cosmos_msg.into();

        Ok(sovereign_msg)
    }
}
