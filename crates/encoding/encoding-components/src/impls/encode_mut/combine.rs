use core::marker::PhantomData;

use cgp::prelude::*;

use crate::traits::decode_mut::MutDecoder;
use crate::traits::encode_mut::MutEncoder;
use crate::traits::types::decode_buffer::HasDecodeBufferType;
use crate::traits::types::encode_buffer::HasEncodeBufferType;

pub struct CombineEncoders<Encoders>(pub PhantomData<Encoders>);

impl<Encoding, Strategy, Encoder, InEncoders, Value> MutEncoder<Encoding, Strategy, Value>
    for CombineEncoders<Cons<Encoder, InEncoders>>
where
    Encoding: HasEncodeBufferType + HasErrorType,
    Encoder: MutEncoder<Encoding, Strategy, Value>,
    CombineEncoders<InEncoders>: MutEncoder<Encoding, Strategy, Value>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        Encoder::encode_mut(encoding, value, buffer)?;
        <CombineEncoders<InEncoders>>::encode_mut(encoding, value, buffer)?;

        Ok(())
    }
}

impl<Encoding, Strategy, Value> MutEncoder<Encoding, Strategy, Value> for CombineEncoders<Nil>
where
    Encoding: HasEncodeBufferType + HasErrorType,
{
    fn encode_mut(
        _encoding: &Encoding,
        _value: &Value,
        _buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        Ok(())
    }
}

impl<Encoding, Strategy, Encoder, InEncoders, ValueA, ValueB>
    MutDecoder<Encoding, Strategy, Cons<ValueA, ValueB>>
    for CombineEncoders<Cons<Encoder, InEncoders>>
where
    Encoding: HasDecodeBufferType + HasErrorType,
    Encoder: MutDecoder<Encoding, Strategy, ValueA>,
    CombineEncoders<InEncoders>: MutDecoder<Encoding, Strategy, ValueB>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Cons<ValueA, ValueB>, Encoding::Error> {
        let value_a = Encoder::decode_mut(encoding, buffer)?;
        let value_b = <CombineEncoders<InEncoders>>::decode_mut(encoding, buffer)?;

        Ok(Cons(value_a, value_b))
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, Nil> for CombineEncoders<Nil>
where
    Encoding: HasDecodeBufferType + HasErrorType,
{
    fn decode_mut(
        _encoding: &Encoding,
        _buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Nil, Encoding::Error> {
        Ok(Nil)
    }
}
