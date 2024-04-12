use cgp_core::HasErrorType;
use hermes_encoding_components::traits::convert::Converter;
use ibc_proto::google::protobuf::Any;
use prost::EncodeError;

use crate::utils::encode::encode_to_any;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoWasmConsensusState {
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

pub struct ProtoConvertWasmConsensusState;

impl<Encoding> Converter<Encoding, WasmConsensusState, ProtoWasmConsensusState>
    for ProtoConvertWasmConsensusState
where
    Encoding: HasErrorType,
{
    fn convert(
        _encoding: &Encoding,
        consensus_state: &WasmConsensusState,
    ) -> Result<ProtoWasmConsensusState, Encoding::Error> {
        Ok(ProtoWasmConsensusState {
            data: consensus_state.data.clone(),
        })
    }
}

impl<Encoding> Converter<Encoding, ProtoWasmConsensusState, WasmConsensusState>
    for ProtoConvertWasmConsensusState
where
    Encoding: HasErrorType,
{
    fn convert(
        _encoding: &Encoding,
        consensus_state: &ProtoWasmConsensusState,
    ) -> Result<WasmConsensusState, Encoding::Error> {
        Ok(WasmConsensusState {
            data: consensus_state.data.clone(),
        })
    }
}

impl WasmConsensusState {
    pub fn encode_protobuf(&self) -> Result<Any, EncodeError> {
        let proto_message = ProtoWasmConsensusState {
            data: self.data.clone(),
        };

        encode_to_any(TYPE_URL, &proto_message)
    }
}
