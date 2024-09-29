use cgp::core::component::UseContext;

use crate::traits::convert::{CanConvert, Converter};
use crate::traits::decode::{CanDecode, Decoder};
use crate::traits::decode_mut::{CanDecodeMut, MutDecoder};
use crate::traits::encode::{CanEncode, Encoder};
use crate::traits::encode_mut::{CanEncodeMut, MutEncoder};

impl<Encoding, Strategy, Value> Encoder<Encoding, Strategy, Value> for UseContext
where
    Encoding: CanEncode<Strategy, Value>,
{
    fn encode(encoding: &Encoding, value: &Value) -> Result<Encoding::Encoded, Encoding::Error> {
        encoding.encode(value)
    }
}

impl<Encoding, Strategy, Value> Decoder<Encoding, Strategy, Value> for UseContext
where
    Encoding: CanDecode<Strategy, Value>,
{
    fn decode(encoding: &Encoding, encoded: &Encoding::Encoded) -> Result<Value, Encoding::Error> {
        encoding.decode(encoded)
    }
}

impl<Encoding, Strategy, Value> MutEncoder<Encoding, Strategy, Value> for UseContext
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

impl<Encoding, Strategy, Value> MutDecoder<Encoding, Strategy, Value> for UseContext
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

impl<Encoding, From, To> Converter<Encoding, From, To> for UseContext
where
    Encoding: CanConvert<From, To>,
{
    fn convert(encoding: &Encoding, from: &From) -> Result<To, Encoding::Error> {
        encoding.convert(from)
    }
}
