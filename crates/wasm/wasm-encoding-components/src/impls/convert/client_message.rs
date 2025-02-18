use cgp::prelude::*;
use hermes_encoding_components::traits::convert::{CanConvert, Converter, ConverterComponent};
use hermes_encoding_components::traits::decode::CanDecode;
use hermes_encoding_components::traits::encode::CanEncode;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use hermes_protobuf_encoding_components::types::strategy::ViaAny;
use ibc::clients::wasm_types::client_message::ClientMessage;
use prost_types::Any;

pub struct EncodeViaClientMessage;

#[cgp_provider(ConverterComponent)]
impl<Encoding, Value> Converter<Encoding, Value, Any> for EncodeViaClientMessage
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanEncode<ViaAny, Value>
        + CanConvert<ClientMessage, Any>,
{
    fn convert(encoding: &Encoding, value: &Value) -> Result<Any, Encoding::Error> {
        let data = encoding.encode(value)?;

        let client_message = ClientMessage { data };

        encoding.convert(&client_message)
    }
}

pub struct DecodeViaClientMessage;

#[cgp_provider(ConverterComponent)]
impl<Encoding, Value> Converter<Encoding, Any, Value> for DecodeViaClientMessage
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanDecode<ViaAny, Value>
        + CanConvert<Any, ClientMessage>,
{
    fn convert(encoding: &Encoding, any: &Any) -> Result<Value, Encoding::Error> {
        let message = encoding.convert(any)?;

        let value = encoding.decode(&message.data)?;

        Ok(value)
    }
}
