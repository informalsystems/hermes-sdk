use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::combine::CombineEncoders;
use hermes_encoding_components::impls::encode_mut::field::EncodeField;
use hermes_encoding_components::impls::encode_mut::with_context::EncodeWithContext;
use hermes_encoding_components::traits::convert::{CanConvert, Converter};
use hermes_encoding_components::traits::decode::{CanDecode, Decoder};
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
use hermes_encoding_components::traits::field::GetField;
use hermes_encoding_components::traits::types::decode_buffer::HasDecodeBufferType;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_encoding_components::HList;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::bytes::EncodeByteField;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::decode_required::DecodeRequiredProtoField;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::encode::EncodeProtoField;
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
                    GetField<symbol!("data")>,
                    EncodeByteField<1>,
                >,
                EncodeField<
                    GetField<symbol!("checksum")>,
                    EncodeByteField<2>,
                >,
                EncodeField<
                    GetField<symbol!("latest_height")>,
                    EncodeProtoField<3, EncodeWithContext>,
                >,
            ]>,
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, WasmClientState> for EncodeWasmClientState
where
    Encoding: HasDecodeBufferType + HasErrorType,
    EncodeByteField<1>: MutDecoder<Encoding, Strategy, Vec<u8>>,
    EncodeByteField<2>: MutDecoder<Encoding, Strategy, Vec<u8>>,
    DecodeRequiredProtoField<3, EncodeWithContext>: MutDecoder<Encoding, Strategy, Height>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<WasmClientState, <Encoding as HasErrorType>::Error> {
        let data = <EncodeByteField<1>>::decode_mut(encoding, buffer)?;
        let checksum = <EncodeByteField<2>>::decode_mut(encoding, buffer)?;
        let latest_height =
            <DecodeRequiredProtoField<3, EncodeWithContext>>::decode_mut(encoding, buffer)?;

        Ok(WasmClientState {
            data,
            checksum,
            latest_height,
        })
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
