use cgp_core::error::{CanRaiseError, HasErrorType};
use hermes_encoding_components::traits::convert::{CanConvert, Converter};
use hermes_encoding_components::traits::decoder::{CanDecode, Decoder};
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_protobuf_encoding_components::types::{Any, ViaAny};
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height;
use ibc_proto::ibc::core::client::v1::Height as ProtoHeight;

#[derive(Clone, Debug)]
pub struct WasmClientState {
    pub data: Vec<u8>,
    pub checksum: Vec<u8>,
    pub latest_height: Height,
}

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoWasmClientState {
    /// bytes encoding the client state of the underlying light client
    /// implemented as a Wasm contract.
    #[prost(bytes = "vec", tag = "1")]
    pub data: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "2")]
    pub checksum: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, optional, tag = "3")]
    pub latest_height: ::core::option::Option<ProtoHeight>,
}

pub struct ProtoConvertWasmClientState;

impl<Encoding> Converter<Encoding, WasmClientState, ProtoWasmClientState>
    for ProtoConvertWasmClientState
where
    Encoding: HasErrorType,
{
    fn convert(
        _encoding: &Encoding,
        client_state: &WasmClientState,
    ) -> Result<ProtoWasmClientState, Encoding::Error> {
        let height = ProtoHeight::from(client_state.latest_height);

        Ok(ProtoWasmClientState {
            data: client_state.data.clone(),
            checksum: client_state.checksum.clone(),
            latest_height: Some(height),
        })
    }
}

impl<Encoding> Converter<Encoding, ProtoWasmClientState, WasmClientState>
    for ProtoConvertWasmClientState
where
    Encoding: CanRaiseError<&'static str> + CanRaiseError<ClientError>,
{
    fn convert(
        _encoding: &Encoding,
        proto_client_state: &ProtoWasmClientState,
    ) -> Result<WasmClientState, Encoding::Error> {
        let proto_client_state = proto_client_state.clone();

        let maybe_height = proto_client_state.latest_height.ok_or_else(|| {
            Encoding::raise_error("Empty 'latest_height' in proto Wasm client state")
        })?;

        let height = Height::try_from(maybe_height).map_err(Encoding::raise_error)?;

        Ok(WasmClientState {
            data: proto_client_state.data,
            checksum: proto_client_state.checksum,
            latest_height: height,
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
