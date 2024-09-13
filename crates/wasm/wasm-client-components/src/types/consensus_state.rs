use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::traits::convert::{CanConvert, Converter};
use hermes_encoding_components::traits::decode::{CanDecode, Decoder};
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode::{CanEncode, Encoder};
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::field::GetField;
use hermes_encoding_components::traits::transform::Transformer;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::bytes::EncodeByteField;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::ViaAny;

#[derive(Debug, HasField)]
pub struct WasmConsensusState {
    pub data: Vec<u8>,
}

pub struct EncodeWasmConsensusState;

delegate_components! {
    EncodeWasmConsensusState {
        MutEncoderComponent:
            EncodeField<
                GetField<symbol!("data")>,
                EncodeByteField<1>,
            >,
        MutDecoderComponent: DecodeFrom<
            Self,
            EncodeByteField<1>,
        >,
    }
}

impl Transformer for EncodeWasmConsensusState {
    type From = Vec<u8>;

    type To = WasmConsensusState;

    fn transform(data: Vec<u8>) -> WasmConsensusState {
        WasmConsensusState { data }
    }
}

pub struct EncodeViaWasmConsensusState;
pub struct DecodeViaWasmConsensusState;

impl<Encoding, Value> Encoder<Encoding, WasmConsensusState, Value> for EncodeViaWasmConsensusState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanEncode<ViaAny, WasmConsensusState>
        + CanEncode<ViaAny, Value>,
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
        + CanDecode<ViaAny, WasmConsensusState>
        + CanDecode<ViaAny, Value>,
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
        + CanEncode<ViaAny, Value>,
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
        + CanDecode<ViaAny, Value>,
{
    fn convert(encoding: &Encoding, any: &Any) -> Result<Value, Encoding::Error> {
        let wasm_consensus_state = encoding.convert(any)?;

        let value: Value = encoding.decode(&wasm_consensus_state.data)?;

        Ok(value)
    }
}
