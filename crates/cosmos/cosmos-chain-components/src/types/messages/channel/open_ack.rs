use ibc::core::client::types::Height;
use ibc::primitives::Signer;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::channel::v1::MsgChannelOpenAck as ProtoMsgChannelOpenAck;

use crate::methods::encode::encode_to_any;
use crate::traits::message::DynCosmosMessage;

const TYPE_URL: &str = "/ibc.core.channel.v1.MsgChannelOpenAck";

#[derive(Debug)]
pub struct CosmosChannelOpenAckMessage {
    pub port_id: String,
    pub channel_id: String,
    pub counterparty_channel_id: String,
    pub counterparty_version: String,
    pub update_height: Height,
    pub proof_try: Vec<u8>,
}

impl DynCosmosMessage for CosmosChannelOpenAckMessage {
    fn counterparty_message_height_for_update_client(&self) -> Option<Height> {
        Some(self.update_height)
    }

    fn encode_protobuf(&self, signer: &Signer) -> Any {
        let proto_message = ProtoMsgChannelOpenAck {
            port_id: self.port_id.clone(),
            channel_id: self.channel_id.clone(),
            counterparty_channel_id: self.counterparty_channel_id.clone(),
            counterparty_version: self.counterparty_version.clone(),
            proof_height: Some(self.update_height.into()),
            proof_try: self.proof_try.clone(),
            signer: signer.to_string(),
        };

        encode_to_any(TYPE_URL, &proto_message)
    }
}
