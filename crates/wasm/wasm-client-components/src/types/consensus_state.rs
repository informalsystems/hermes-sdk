use cgp_core::error::HasErrorType;
use hermes_encoding_components::traits::convert::{CanConvert, Converter};
use hermes_encoding_components::traits::decoder::{CanDecode, Decoder};
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::encoder::{CanEncode, Encoder};
use hermes_protobuf_encoding_components::types::Any;

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoWasmConsensusState {
    /// bytes encoding the consensus state of the underlying light client
    /// implemented as a Wasm contract.
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}

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

pub struct EncodeViaWasmConsensusState;
pub struct DecodeViaWasmConsensusState;

impl<Encoding, Value> Encoder<Encoding, WasmConsensusState, Value> for EncodeViaWasmConsensusState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanEncode<Any, WasmConsensusState>
        + CanEncode<Any, Value>,
{
    fn encode(encoding: &Encoding, value: &Value) -> Result<Vec<u8>, Encoding::Error> {
        let data = encoding.encode(value)?;

        let consensus_state = WasmConsensusState { data };

        encoding.encode(&consensus_state)
    }
}

impl<Encoding, Value> Decoder<Encoding, WasmConsensusState, Value> for EncodeViaWasmConsensusState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanDecode<Any, WasmConsensusState>
        + CanDecode<Any, Value>,
{
    fn decode(encoding: &Encoding, encoded: &Vec<u8>) -> Result<Value, Encoding::Error> {
        let wasm_client_state: WasmConsensusState = encoding.decode(encoded)?;

        let value: Value = encoding.decode(&wasm_client_state.data)?;

        Ok(value)
    }
}

impl<Encoding, Value> Converter<Encoding, Value, Any> for EncodeViaWasmConsensusState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanConvert<WasmConsensusState, Any>
        + CanEncode<Any, Value>,
{
    fn convert(encoding: &Encoding, value: &Value) -> Result<Any, Encoding::Error> {
        let data = encoding.encode(value)?;

        let any = encoding.convert(&WasmConsensusState { data })?;

        Ok(any)
    }
}

impl<Encoding, Value> Converter<Encoding, Any, Value> for DecodeViaWasmConsensusState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanConvert<Any, WasmConsensusState>
        + CanDecode<Any, Value>,
{
    fn convert(encoding: &Encoding, any: &Any) -> Result<Value, Encoding::Error> {
        let wasm_consensus_state = encoding.convert(any)?;

        let value: Value = encoding.decode(&wasm_consensus_state.data)?;

        Ok(value)
    }
}
