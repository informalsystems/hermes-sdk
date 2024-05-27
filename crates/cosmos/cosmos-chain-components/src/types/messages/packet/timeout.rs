use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::channel::v1::MsgTimeout as ProtoMsgTimeout;
use ibc_relayer_types::core::ics04_channel::packet::{Packet, Sequence};
use ibc_relayer_types::signer::Signer;
use ibc_relayer_types::Height;

use crate::methods::encode::encode_to_any;
use crate::traits::message::DynCosmosMessage;

const TYPE_URL: &str = "/ibc.core.channel.v1.MsgTimeout";

#[derive(Debug)]
pub struct CosmosTimeoutPacketMessage {
    pub packet: Packet,
    pub next_sequence_recv: Sequence,
    pub update_height: Height,
    pub proof_unreceived: Vec<u8>,
}

impl DynCosmosMessage for CosmosTimeoutPacketMessage {
    fn counterparty_message_height_for_update_client(&self) -> Option<Height> {
        Some(self.update_height)
    }

    fn encode_protobuf(&self, signer: &Signer) -> Any {
        let proto_message = ProtoMsgTimeout {
            packet: Some(self.packet.clone().into()),
            next_sequence_recv: self.next_sequence_recv.into(),
            proof_unreceived: self.proof_unreceived.clone(),
            proof_height: Some(self.update_height.into()),
            signer: signer.to_string(),
        };

        encode_to_any(TYPE_URL, &proto_message)
    }
}
