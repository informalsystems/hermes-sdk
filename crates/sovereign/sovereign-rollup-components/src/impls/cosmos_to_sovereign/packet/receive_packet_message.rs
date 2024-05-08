use cgp_core::HasErrorType;

use hermes_cosmos_chain_components::traits::message::ToCosmosMessage;
use hermes_cosmos_chain_components::types::messages::packet::receive::CosmosReceivePacketMessage;
use hermes_cosmos_chain_components::types::payloads::packet::CosmosReceivePacketPayload;
use hermes_relayer_components::chain::traits::message_builders::receive_packet::ReceivePacketMessageBuilder;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::chain::traits::types::packets::receive::HasReceivePacketPayloadType;
use ibc_relayer_types::core::ics04_channel::packet::Packet;

use crate::types::message::SovereignMessage;

pub struct BuildReceivePacketMessageOnSovereign;

impl<Rollup, Counterparty> ReceivePacketMessageBuilder<Rollup, Counterparty>
    for BuildReceivePacketMessageOnSovereign
where
    Rollup: HasMessageType<Message = SovereignMessage>
        + HasIbcPacketTypes<Counterparty, IncomingPacket = Packet>
        + HasErrorType,
    Counterparty:
        HasReceivePacketPayloadType<Rollup, ReceivePacketPayload = CosmosReceivePacketPayload>,
{
    async fn build_receive_packet_message(
        _rollup: &Rollup,
        packet: &Packet,
        payload: CosmosReceivePacketPayload,
    ) -> Result<SovereignMessage, Rollup::Error> {
        let msg = CosmosReceivePacketMessage {
            packet: packet.clone(),
            update_height: payload.update_height,
            proof_commitment: payload.proof_commitment,
        };

        let cosmos_msg = msg.to_cosmos_message();
        let sovereign_msg: SovereignMessage = cosmos_msg.into();

        Ok(sovereign_msg)
    }
}
