use cgp_core::{CanRaiseError, HasErrorType};
use hermes_relayer_components::encode::decoder::Decoder;
use hermes_relayer_components::encode::encoded::HasEncodedType;
use hermes_relayer_components::encode::encoder::Encoder;
use prost::{DecodeError, Message};

pub struct EncodeAsProtobuf;

impl<Encoding, Value> Encoder<Encoding, Value> for EncodeAsProtobuf
where
    Value: Message + Default,
    Encoding: HasEncodedType<Encoded = Vec<u8>> + HasErrorType,
{
    fn encode(_encoding: &Encoding, value: &Value) -> Result<Vec<u8>, Encoding::Error> {
        Ok(value.encode_to_vec())
    }
}

impl<Encoding, Value> Decoder<Encoding, Value> for EncodeAsProtobuf
where
    Value: Message + Default,
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanRaiseError<DecodeError>,
{
    fn decode(_encoding: &Encoding, encoded: &Vec<u8>) -> Result<Value, Encoding::Error> {
        let decoded = Message::decode(encoded.as_ref()).map_err(Encoding::raise_error)?;

        Ok(decoded)
    }
}
