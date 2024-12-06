use hermes_error::types::Error;
use ibc_proto::google::protobuf::Any;
pub use ibc_proto::ibc::lightclients::solomachine::v3::ConsensusState as ProtoSolomachineConsensusState;
use ibc_proto::Protobuf;
use prost::Message;

use crate::methods::encode::public_key::{
    decode_public_key_from_any, encode_public_key, PublicKey,
};

pub const SOLOMACHINE_CONSENSUS_STATE_TYPE_URL: &str =
    "/ibc.lightclients.solomachine.v3.ConsensusState";

#[derive(Clone, Debug)]
pub struct SolomachineConsensusState {
    pub public_key: Option<PublicKey>,
    pub diversifier: String,
    pub timestamp: u64,
}

/*impl Msg for SolomachineConsensusState {
    type ValidationError = Error;
    type Raw = ProtoSolomachineConsensusState;

    fn route(&self) -> String {
        "ibc".to_string()
    }

    fn type_url(&self) -> String {
        SOLOMACHINE_CONSENSUS_STATE_TYPE_URL.to_string()
    }
}*/

impl Protobuf<ProtoSolomachineConsensusState> for SolomachineConsensusState {}

impl TryFrom<ProtoSolomachineConsensusState> for SolomachineConsensusState {
    type Error = Error;

    fn try_from(value: ProtoSolomachineConsensusState) -> Result<Self, Self::Error> {
        let pk = value.public_key.map(decode_public_key_from_any);

        Ok(SolomachineConsensusState {
            public_key: pk,
            diversifier: value.diversifier,
            timestamp: value.timestamp,
        })
    }
}

impl From<SolomachineConsensusState> for ProtoSolomachineConsensusState {
    fn from(value: SolomachineConsensusState) -> Self {
        let pk = value.public_key.map(|key| encode_public_key(&key));
        ProtoSolomachineConsensusState {
            public_key: pk,
            diversifier: value.diversifier,
            timestamp: value.timestamp,
        }
    }
}

pub fn decode_client_consensus_state(buf: &[u8]) -> SolomachineConsensusState {
    let any_value = Any::decode(buf).unwrap();
    let proto_state = ProtoSolomachineConsensusState::decode(any_value.value.as_ref()).unwrap();

    let client_consensus_state = proto_state.try_into().unwrap();

    client_consensus_state
}
