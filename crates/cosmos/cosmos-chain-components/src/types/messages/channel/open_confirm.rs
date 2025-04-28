use ibc::core::client::types::Height;
use ibc::primitives::Signer;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::channel::v1::MsgChannelOpenConfirm as ProtoMsgChannelOpenConfirm;

use crate::methods::encode_to_any;
use crate::traits::DynCosmosMessage;

const TYPE_URL: &str = "/ibc.core.channel.v1.MsgChannelOpenConfirm";

#[derive(Debug)]
pub struct CosmosChannelOpenConfirmMessage {
    pub port_id: String,
    pub channel_id: String,
    pub update_height: Height,
    pub proof_ack: Vec<u8>,
}

impl DynCosmosMessage for CosmosChannelOpenConfirmMessage {
    fn counterparty_message_height_for_update_client(&self) -> Option<Height> {
        Some(self.update_height)
    }

    fn encode_protobuf(&self, signer: &Signer) -> Any {
        let proto_message = ProtoMsgChannelOpenConfirm {
            port_id: self.port_id.clone(),
            channel_id: self.channel_id.clone(),
            proof_height: Some(self.update_height.into()),
            proof_ack: self.proof_ack.clone(),
            signer: signer.to_string(),
        };

        encode_to_any(TYPE_URL, &proto_message)
    }
}
