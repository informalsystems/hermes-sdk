use ibc::core::channel::types::packet::Packet;
use ibc::core::client::types::Height;
use ibc::primitives::Signer;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::channel::v1::MsgRecvPacket as ProtoMsgRecvPacket;

use crate::methods::encode_to_any;
use crate::traits::DynCosmosMessage;

const TYPE_URL: &str = "/ibc.core.channel.v1.MsgRecvPacket";

#[derive(Debug)]
pub struct CosmosReceivePacketMessage {
    pub packet: Packet,
    pub update_height: Height,
    pub proof_commitment: Vec<u8>,
}

impl DynCosmosMessage for CosmosReceivePacketMessage {
    fn counterparty_message_height_for_update_client(&self) -> Option<Height> {
        Some(self.update_height)
    }

    fn encode_protobuf(&self, signer: &Signer) -> Any {
        let proto_message = ProtoMsgRecvPacket {
            packet: Some(self.packet.clone().into()),
            proof_commitment: self.proof_commitment.clone(),
            proof_height: Some(self.update_height.into()),
            signer: signer.to_string(),
        };

        encode_to_any(TYPE_URL, &proto_message)
    }
}
