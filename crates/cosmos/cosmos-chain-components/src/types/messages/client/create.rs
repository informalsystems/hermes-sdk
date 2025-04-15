use ibc::primitives::Signer;
use ibc_proto::google::protobuf::Any as IbcProtoAny;
use prost::Message;
use prost_types::Any;

use crate::methods::encode_to_any;
use crate::traits::DynCosmosMessage;

const TYPE_URL: &str = "/ibc.core.client.v1.MsgCreateClient";

#[derive(Debug)]
pub struct CosmosCreateClientMessage {
    pub client_state: Any,
    pub consensus_state: Any,
}

#[derive(Message)]
pub struct ProtoMsgCreateClient {
    /// light client state
    #[prost(message, optional, tag = "1")]
    pub client_state: Option<Any>,
    /// consensus state associated with the client that corresponds to a given
    /// height.
    #[prost(message, optional, tag = "2")]
    pub consensus_state: Option<Any>,
    /// signer address
    #[prost(string, tag = "3")]
    pub signer: String,
}

impl DynCosmosMessage for CosmosCreateClientMessage {
    fn encode_protobuf(&self, signer: &Signer) -> IbcProtoAny {
        let proto_message = ProtoMsgCreateClient {
            client_state: Some(self.client_state.clone()),
            consensus_state: Some(self.consensus_state.clone()),
            signer: signer.to_string(),
        };

        encode_to_any(TYPE_URL, &proto_message)
    }
}
