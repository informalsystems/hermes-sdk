use cgp::core::component::UseContext;
use hermes_prelude::*;

use crate::traits::{
    CanConvert, CanDecode, CanDecodeMut, CanEncode, CanEncodeMut, Converter, ConverterComponent,
    Decoder, DecoderComponent, Encoder, EncoderComponent, MutDecoder, MutDecoderComponent,
    MutEncoder, MutEncoderComponent,
};

#[cgp_provider(EncoderComponent)]
impl<Encoding, Strategy, Value> Encoder<Encoding, Strategy, Value> for UseContext
where
    Encoding: CanEncode<Strategy, Value>,
{
    fn encode(encoding: &Encoding, value: &Value) -> Result<Encoding::Encoded, Encoding::Error> {
        encoding.encode(value)
    }
}

#[cgp_provider(DecoderComponent)]
impl<Encoding, Strategy, Value> Decoder<Encoding, Strategy, Value> for UseContext
where
    Encoding: CanDecode<Strategy, Value>,
{
    fn decode(encoding: &Encoding, encoded: &Encoding::Encoded) -> Result<Value, Encoding::Error> {
        encoding.decode(encoded)
    }
}

#[cgp_provider(MutEncoderComponent)]
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

#[cgp_provider(MutDecoderComponent)]
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

#[cgp_provider(ConverterComponent)]
impl<Encoding, From, To> Converter<Encoding, From, To> for UseContext
where
    Encoding: CanConvert<From, To>,
{
    fn convert(encoding: &Encoding, from: &From) -> Result<To, Encoding::Error> {
        encoding.convert(from)
    }
}
