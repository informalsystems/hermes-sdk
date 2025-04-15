use core::time::Duration;

use ibc::core::client::types::Height;
use ibc::core::connection::types::version::Version;
use ibc::primitives::Signer;
use ibc_proto::google::protobuf::Any as IbcProtoAny;
use ibc_proto::ibc::core::commitment::v1::MerklePrefix;
use ibc_proto::ibc::core::connection::v1::{
    Counterparty, MsgConnectionOpenTry as ProtoMsgConnectionOpenTry,
};
use prost_types::Any;

use crate::methods::encode_to_any;
use crate::traits::DynCosmosMessage;

const TYPE_URL: &str = "/ibc.core.connection.v1.MsgConnectionOpenTry";

#[derive(Debug)]
pub struct CosmosConnectionOpenTryMessage {
    pub client_id: String,
    pub counterparty_client_id: String,
    pub counterparty_connection_id: String,
    pub counterparty_commitment_prefix: Vec<u8>,
    pub counterparty_versions: Vec<Version>,
    pub client_state: Any,
    pub delay_period: Duration,
    pub update_height: Height,
    pub proof_init: Vec<u8>,
    pub proof_client: Vec<u8>,
    pub proof_consensus: Vec<u8>,
    pub proof_consensus_height: Height,
}

impl DynCosmosMessage for CosmosConnectionOpenTryMessage {
    fn counterparty_message_height_for_update_client(&self) -> Option<Height> {
        Some(self.update_height)
    }

    fn encode_protobuf(&self, signer: &Signer) -> IbcProtoAny {
        let counterparty = Counterparty {
            client_id: self.counterparty_client_id.clone(),
            prefix: Some(MerklePrefix {
                key_prefix: self.counterparty_commitment_prefix.clone(),
            }),
            connection_id: self.counterparty_connection_id.clone(),
        };

        #[allow(deprecated)]
        let proto_message = ProtoMsgConnectionOpenTry {
            client_id: self.client_id.clone(),
            counterparty: Some(counterparty),
            counterparty_versions: self
                .counterparty_versions
                .iter()
                .map(|v| v.clone().into())
                .collect(),
            client_state: Some(IbcProtoAny {
                type_url: self.client_state.type_url.clone(),
                value: self.client_state.value.clone(),
            }),
            delay_period: self.delay_period.as_nanos() as u64,
            proof_height: Some(self.update_height.into()),
            proof_init: self.proof_init.clone(),
            proof_client: self.proof_client.clone(),
            proof_consensus: self.proof_consensus.clone(),
            consensus_height: Some(self.proof_consensus_height.into()),
            signer: signer.to_string(),
            previous_connection_id: "".to_string(),
            host_consensus_state_proof: Vec::new(),
        };

        encode_to_any(TYPE_URL, &proto_message)
    }
}
