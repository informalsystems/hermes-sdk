use core::marker::PhantomData;

use cgp::prelude::*;

use crate::traits::decode_mut::{MutDecoder, MutDecoderComponent};
use crate::traits::encode_mut::{MutEncoder, MutEncoderComponent};
use crate::traits::types::decode_buffer::HasDecodeBufferType;
use crate::traits::types::encode_buffer::HasEncodeBufferType;

pub struct CombineEncoders<Encoders>(pub PhantomData<Encoders>);

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, Encoder, InEncoders, Value> MutEncoder<Encoding, Strategy, Value>
    for CombineEncoders<Cons<Encoder, InEncoders>>
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
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

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, Value> MutEncoder<Encoding, Strategy, Value> for CombineEncoders<Nil>
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
{
    fn encode_mut(
        _encoding: &Encoding,
        _value: &Value,
        _buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, Encoder, InEncoders, ValueA, ValueB>
    MutDecoder<Encoding, Strategy, Cons<ValueA, ValueB>>
    for CombineEncoders<Cons<Encoder, InEncoders>>
where
    Encoding: HasDecodeBufferType + HasAsyncErrorType,
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

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, Nil> for CombineEncoders<Nil>
where
    Encoding: HasDecodeBufferType + HasAsyncErrorType,
{
    fn decode_mut(
        _encoding: &Encoding,
        _buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Nil, Encoding::Error> {
        Ok(Nil)
    }
}
