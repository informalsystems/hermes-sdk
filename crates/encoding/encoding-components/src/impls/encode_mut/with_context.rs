use crate::traits::decode_mut::{CanDecodeMut, MutDecoder};
use crate::traits::encode_mut::{CanEncodeMut, MutEncoder};

pub struct EncodeWithContext;

impl<Encoding, Strategy, Value> MutEncoder<Encoding, Strategy, Value> for EncodeWithContext
where
    Encoding: CanEncodeMut<Strategy, Value>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encoding.encode_mut(value, buffer)
    }
}

impl<Encoding, Strategy, Value> MutDecoder<Encoding, Strategy, Value> for EncodeWithContext
where
    Encoding: CanDecodeMut<Strategy, Value>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Value, Encoding::Error> {
        encoding.decode_mut(buffer)
    }
}
