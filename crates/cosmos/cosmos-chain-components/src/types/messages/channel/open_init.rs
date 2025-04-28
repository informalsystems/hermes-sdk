use ibc::primitives::Signer;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::channel::v1::{Channel, MsgChannelOpenInit as ProtoMsgChannelOpenInit};

use crate::methods::encode_to_any;
use crate::traits::DynCosmosMessage;

const TYPE_URL: &str = "/ibc.core.channel.v1.MsgChannelOpenInit";

#[derive(Debug)]
pub struct CosmosChannelOpenInitMessage {
    pub port_id: String,
    pub channel: Channel,
}

impl DynCosmosMessage for CosmosChannelOpenInitMessage {
    fn encode_protobuf(&self, signer: &Signer) -> Any {
        let proto_message = ProtoMsgChannelOpenInit {
            port_id: self.port_id.clone(),
            channel: Some(self.channel.clone()),
            signer: signer.to_string(),
        };

        encode_to_any(TYPE_URL, &proto_message)
    }
}
