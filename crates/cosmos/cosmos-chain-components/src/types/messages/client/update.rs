use ibc::core::host::types::identifiers::ClientId;
use ibc::primitives::Signer;
use ibc_proto::google::protobuf::Any as IbcProtoAny;
use prost::Message;
use prost_types::Any;

use crate::methods::encode_to_any;
use crate::traits::DynCosmosMessage;

const TYPE_URL: &str = "/ibc.core.client.v1.MsgUpdateClient";

#[derive(Debug)]
pub struct CosmosUpdateClientMessage {
    pub client_id: ClientId,
    pub header: Any,
}

#[derive(Message)]
pub struct ProtoMsgUpdateClient {
    /// client unique identifier
    #[prost(string, tag = "1")]
    pub client_id: String,
    /// client message to update the light client
    #[prost(message, optional, tag = "2")]
    pub client_message: Option<Any>,
    /// signer address
    #[prost(string, tag = "3")]
    pub signer: String,
}

impl DynCosmosMessage for CosmosUpdateClientMessage {
    fn encode_protobuf(&self, signer: &Signer) -> IbcProtoAny {
        let proto_message = ProtoMsgUpdateClient {
            client_id: self.client_id.to_string(),
            client_message: Some(self.header.clone()),
            signer: signer.to_string(),
        };

        encode_to_any(TYPE_URL, &proto_message)
    }
}
