use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::client::v1::MsgUpdateClient as ProtoMsgUpdateClient;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::signer::Signer;

use crate::methods::encode::encode_to_any;
use crate::traits::message::DynCosmosMessage;

pub const TYPE_URL: &str = "/ibc.core.client.v1.MsgUpdateClient";

#[derive(Debug)]
pub struct CosmosUpdateClientMessage {
    pub client_id: ClientId,
    pub header: Any,
}

impl DynCosmosMessage for CosmosUpdateClientMessage {
    fn encode_protobuf(&self, signer: &Signer) -> Any {
        let proto_message = ProtoMsgUpdateClient {
            client_id: self.client_id.to_string(),
            client_message: Some(self.header.clone()),
            signer: signer.to_string(),
        };

        encode_to_any(TYPE_URL, &proto_message)
    }
}
