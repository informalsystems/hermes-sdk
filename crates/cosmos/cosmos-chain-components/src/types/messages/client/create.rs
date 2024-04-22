use ibc_proto::google::protobuf::Any as IbcProtoAny;
use ibc_proto::ibc::core::client::v1::MsgCreateClient as ProtoMsgCreateClient;
use ibc_relayer_types::signer::Signer;
use prost::EncodeError;
use prost_types::Any;

use crate::methods::encode::encode_to_any;
use crate::traits::message::DynCosmosMessage;

const TYPE_URL: &str = "/ibc.core.client.v1.MsgCreateClient";

#[derive(Debug)]
pub struct CosmosCreateClientMessage {
    pub client_state: Any,
    pub consensus_state: Any,
}

impl DynCosmosMessage for CosmosCreateClientMessage {
    fn encode_protobuf(&self, signer: &Signer) -> Result<IbcProtoAny, EncodeError> {
        let proto_message = ProtoMsgCreateClient {
            client_state: Some(IbcProtoAny {
                type_url: self.client_state.type_url.clone(),
                value: self.client_state.value.clone(),
            }),
            consensus_state: Some(IbcProtoAny {
                type_url: self.consensus_state.type_url.clone(),
                value: self.consensus_state.value.clone(),
            }),
            signer: signer.to_string(),
        };

        encode_to_any(TYPE_URL, &proto_message)
    }
}
