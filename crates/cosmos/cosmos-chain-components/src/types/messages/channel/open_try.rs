use ibc::core::client::types::Height;
use ibc::primitives::Signer;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::channel::v1::{Channel, MsgChannelOpenTry as ProtoMsgChannelOpenTry};

use crate::methods::encode_to_any;
use crate::traits::DynCosmosMessage;

const TYPE_URL: &str = "/ibc.core.channel.v1.MsgChannelOpenTry";

#[derive(Debug)]
pub struct CosmosChannelOpenTryMessage {
    pub port_id: String,
    pub channel: Channel,
    pub counterparty_version: String,
    pub update_height: Height,
    pub proof_init: Vec<u8>,
}

impl DynCosmosMessage for CosmosChannelOpenTryMessage {
    fn counterparty_message_height_for_update_client(&self) -> Option<Height> {
        Some(self.update_height)
    }

    fn encode_protobuf(&self, signer: &Signer) -> Any {
        #[allow(deprecated)]
        let proto_message = ProtoMsgChannelOpenTry {
            port_id: self.port_id.clone(),
            channel: Some(self.channel.clone()),
            counterparty_version: self.counterparty_version.clone(),
            proof_height: Some(self.update_height.into()),
            proof_init: self.proof_init.clone(),
            signer: signer.to_string(),
            previous_channel_id: "".to_string(),
        };

        encode_to_any(TYPE_URL, &proto_message)
    }
}
