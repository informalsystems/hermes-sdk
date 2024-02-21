use ibc_proto::google::protobuf::Any;
use prost::EncodeError;

use crate::wasm::types::messages::utils::encode::encode_to_any;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoConsensusState {
    /// bytes encoding the consensus state of the underlying light client
    /// implemented as a Wasm contract.
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}

const TYPE_URL: &str = "/ibc.lightclients.wasm.v1.ConsensusState";

#[derive(Debug)]
pub struct WasmConsensusState {
    pub data: Vec<u8>,
}

impl WasmConsensusState {
    pub fn encode_protobuf(&self) -> Result<Any, EncodeError> {
        let proto_message = ProtoConsensusState {
            data: self.data.clone(),
        };

        encode_to_any(TYPE_URL, &proto_message)
    }
}
