use ibc::core::channel::types::packet::Packet;
use ibc::core::client::types::Height;
use ibc::primitives::Signer;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::channel::v1::MsgAcknowledgement as ProtoMsgAcknowledgement;

use crate::methods::encode::encode_to_any;
use crate::traits::message::DynCosmosMessage;

const TYPE_URL: &str = "/ibc.core.channel.v1.MsgAcknowledgement";

#[derive(Debug)]
pub struct CosmosAckPacketMessage {
    pub packet: Packet,
    pub acknowledgement: Vec<u8>,
    pub update_height: Height,
    pub proof_acked: Vec<u8>,
}

impl DynCosmosMessage for CosmosAckPacketMessage {
    fn counterparty_message_height_for_update_client(&self) -> Option<Height> {
        Some(self.update_height)
    }

    fn encode_protobuf(&self, signer: &Signer) -> Any {
        let proto_message = ProtoMsgAcknowledgement {
            packet: Some(self.packet.clone().into()),
            acknowledgement: self.acknowledgement.clone(),
            proof_acked: self.proof_acked.clone(),
            proof_height: Some(self.update_height.into()),
            signer: signer.to_string(),
        };

        encode_to_any(TYPE_URL, &proto_message)
    }
}
