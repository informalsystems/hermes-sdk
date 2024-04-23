use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::channel::v1::MsgChannelOpenInit as ProtoMsgChannelOpenInit;
use ibc_relayer_types::core::ics04_channel::channel::ChannelEnd;
use ibc_relayer_types::core::ics24_host::identifier::PortId;
use ibc_relayer_types::signer::Signer;

use crate::methods::encode::encode_to_any;
use crate::traits::message::DynCosmosMessage;

const TYPE_URL: &str = "/ibc.core.channel.v1.MsgChannelOpenInit";

#[derive(Debug)]
pub struct CosmosChannelOpenInitMessage {
    pub port_id: PortId,
    pub channel: ChannelEnd,
}

impl DynCosmosMessage for CosmosChannelOpenInitMessage {
    fn encode_protobuf(&self, signer: &Signer) -> Any {
        let proto_message = ProtoMsgChannelOpenInit {
            port_id: self.port_id.to_string(),
            channel: Some(self.channel.clone().into()),
            signer: signer.to_string(),
        };

        encode_to_any(TYPE_URL, &proto_message)
    }
}
