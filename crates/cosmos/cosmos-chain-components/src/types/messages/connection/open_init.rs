use core::time::Duration;

use ibc::core::connection::types::version::Version;
use ibc::core::host::types::identifiers::ClientId;
use ibc::primitives::Signer;
use ibc_proto::google::protobuf::Any;
use ibc_proto::ibc::core::commitment::v1::MerklePrefix;
use ibc_proto::ibc::core::connection::v1::{
    Counterparty, MsgConnectionOpenInit as ProtoMsgConnectionOpenInit,
};

use crate::methods::encode::encode_to_any;
use crate::traits::message::DynCosmosMessage;

const TYPE_URL: &str = "/ibc.core.connection.v1.MsgConnectionOpenInit";

#[derive(Debug)]
pub struct CosmosConnectionOpenInitMessage {
    pub client_id: ClientId,
    pub counterparty_client_id: ClientId,
    pub counterparty_commitment_prefix: Vec<u8>,
    pub version: Version,
    pub delay_period: Duration,
}

impl DynCosmosMessage for CosmosConnectionOpenInitMessage {
    fn encode_protobuf(&self, signer: &Signer) -> Any {
        let counterparty = Counterparty {
            client_id: self.counterparty_client_id.as_str().to_string(),
            prefix: Some(MerklePrefix {
                key_prefix: self.counterparty_commitment_prefix.clone(),
            }),
            connection_id: "".to_string(),
        };

        let proto_message = ProtoMsgConnectionOpenInit {
            client_id: self.client_id.as_str().to_string(),
            counterparty: Some(counterparty),
            version: Some(self.version.clone().into()),
            delay_period: self.delay_period.as_nanos() as u64,
            signer: signer.to_string(),
        };

        encode_to_any(TYPE_URL, &proto_message)
    }
}
