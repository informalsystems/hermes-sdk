use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::from::DecodeFrom;
use hermes_encoding_components::impls::from_context::EncodeFromContext;
use hermes_encoding_components::traits::convert::{CanConvert, Converter};
use hermes_encoding_components::traits::decode::{CanDecode, Decoder};
use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::transform::Transformer;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_encoding_components::HList;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::bytes::EncodeByteField;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::decode_required::DecodeRequiredProtoField;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::encode::EncodeLengthDelimitedProtoField;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::ViaAny;
use ibc::core::client::types::Height;

#[derive(Clone, Debug, HasField, PartialEq, Eq)]
pub struct WasmClientState {
    pub data: Vec<u8>,
    pub checksum: Vec<u8>,
    pub latest_height: Height,
}

pub struct EncodeWasmClientState;

delegate_components! {
    EncodeWasmClientState {
        MutEncoderComponent:
            CombineEncoders<HList![
                EncodeField<
                    symbol!("data"),
                    EncodeByteField<1>,
                >,
                EncodeField<
                    symbol!("checksum"),
                    EncodeByteField<2>,
                >,
                EncodeField<
                    symbol!("latest_height"),
                    EncodeLengthDelimitedProtoField<3, EncodeFromContext>,
                >,
            ]>,
        MutDecoderComponent: DecodeFrom<
            Self,
            CombineEncoders<HList![
                EncodeByteField<1>,
                EncodeByteField<2>,
                DecodeRequiredProtoField<3, EncodeFromContext>,
            ]>
        >,
    }
}

impl Transformer for EncodeWasmClientState {
    type From = HList![Vec<u8>, Vec<u8>, Height];

    type To = WasmClientState;

    fn transform(HList![data, checksum, latest_height]: Self::From) -> Self::To {
        WasmClientState {
            data,
            checksum,
            latest_height,
        }
    }
}

pub struct DecodeViaWasmClientState;

impl<Encoding, Value> Converter<Encoding, Any, Value> for DecodeViaWasmClientState
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanConvert<Any, WasmClientState>
        + CanDecode<ViaAny, Value>,
{
    fn convert(encoding: &Encoding, any: &Any) -> Result<Value, Encoding::Error> {
        let wasm_client_state = encoding.convert(any)?;

        let value: Value = encoding.decode(&wasm_client_state.data)?;

        Ok(value)
    }
}

impl<Encoding, Value> Decoder<Encoding, WasmClientState, Value> for DecodeViaWasmClientState
where
    Encoding:
        HasEncodedType<Encoded = Vec<u8>> + CanDecode<Any, WasmClientState> + CanDecode<Any, Value>,
{
    fn decode(encoding: &Encoding, encoded: &Vec<u8>) -> Result<Value, Encoding::Error> {
        let wasm_client_state: WasmClientState = encoding.decode(encoded)?;

        let value: Value = encoding.decode(&wasm_client_state.data)?;

        Ok(value)
    }
}
