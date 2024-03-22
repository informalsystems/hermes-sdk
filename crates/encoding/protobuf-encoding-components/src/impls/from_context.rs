use hermes_encoding_components::traits::decoder::{CanDecode, Decoder};
use hermes_encoding_components::traits::encoder::{CanEncode, Encoder};
pub struct EncodeFromContext;

impl<Encoding, Value> Encoder<Encoding, Value> for EncodeFromContext
where
    Encoding: CanEncode<Value>,
{
    fn encode(encoding: &Encoding, value: &Value) -> Result<Encoding::Encoded, Encoding::Error> {
        encoding.encode(value)
    }
}

impl<Encoding, Value> Decoder<Encoding, Value> for EncodeFromContext
where
    Encoding: CanDecode<Value>,
{
    fn decode(encoding: &Encoding, encoded: &Encoding::Encoded) -> Result<Value, Encoding::Error> {
        encoding.decode(encoded)
    }
}
