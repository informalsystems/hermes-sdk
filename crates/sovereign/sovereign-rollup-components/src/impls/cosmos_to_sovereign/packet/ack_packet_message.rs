use cgp_core::prelude::*;
use cgp_core::HasErrorType;

use hermes_cosmos_chain_components::methods::encode::encode_to_any;
use hermes_cosmos_chain_components::types::messages::packet::ack::TYPE_URL;
use hermes_cosmos_chain_components::types::payloads::packet::CosmosAckPacketPayload;
use hermes_relayer_components::chain::traits::message_builders::ack_packet::AckPacketMessageBuilder;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use hermes_relayer_components::chain::traits::types::packets::ack::HasAckPacketPayloadType;
use ibc_proto::ibc::core::channel::v1::MsgAcknowledgement as ProtoMsgAcknowledgement;
use ibc_relayer_types::core::ics04_channel::packet::Packet;
use ibc_relayer_types::signer::Signer;

use crate::types::message::SovereignMessage;
use crate::types::messages::ibc::IbcMessage;

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
        let proto_message = ProtoMsgAcknowledgement {
            packet: Some(packet.clone().into()),
            acknowledgement: payload.ack,
            proof_acked: payload.proof_acked.into(),
            proof_height: Some(payload.update_height.into()),
            signer: Signer::dummy().to_string(),
        };

        let any_message = encode_to_any(TYPE_URL, &proto_message);

        let message = SovereignMessage::Ibc(IbcMessage::Core(any_message));

        Ok(message)
    }
}
