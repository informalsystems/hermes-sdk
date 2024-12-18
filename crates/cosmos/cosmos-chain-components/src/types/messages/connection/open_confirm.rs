use ibc::core::client::types::Height;
use ibc::primitives::Signer;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::connection::v1::MsgConnectionOpenConfirm as ProtoMsgConnectionOpenConfirm;

use crate::methods::encode::encode_to_any;
use crate::traits::message::DynCosmosMessage;

const TYPE_URL: &str = "/ibc.core.connection.v1.MsgConnectionOpenConfirm";

#[derive(Debug)]
pub struct CosmosConnectionOpenConfirmMessage {
    pub connection_id: String,
    pub update_height: Height,
    pub proof_ack: Vec<u8>,
}

impl DynCosmosMessage for CosmosConnectionOpenConfirmMessage {
    fn counterparty_message_height_for_update_client(&self) -> Option<Height> {
        Some(self.update_height)
    }

    fn encode_protobuf(&self, signer: &Signer) -> Any {
        let proto_message = ProtoMsgConnectionOpenConfirm {
            connection_id: self.connection_id.clone(),
            proof_height: Some(self.update_height.into()),
            proof_ack: self.proof_ack.clone(),
            signer: signer.to_string(),
        };

        encode_to_any(TYPE_URL, &proto_message)
    }
}
