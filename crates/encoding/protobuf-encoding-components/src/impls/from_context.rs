use hermes_encoding_components::traits::decoder::{CanDecode, Decoder};
use hermes_encoding_components::traits::encoder::{CanEncode, Encoder};
pub struct EncodeFromContext;

impl<Encoding, Strategy, Value> Encoder<Encoding, Strategy, Value> for EncodeFromContext
where
    Encoding: CanEncode<Strategy, Value>,
{
    fn encode(encoding: &Encoding, value: &Value) -> Result<Encoding::Encoded, Encoding::Error> {
        encoding.encode(value)
    }
}

impl<Encoding, Strategy, Value> Decoder<Encoding, Strategy, Value> for EncodeFromContext
where
    Encoding: CanDecode<Strategy, Value>,
{
    fn decode(encoding: &Encoding, encoded: &Encoding::Encoded) -> Result<Value, Encoding::Error> {
        encoding.decode(encoded)
    }
}
