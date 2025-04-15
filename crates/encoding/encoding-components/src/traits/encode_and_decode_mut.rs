use cgp::core::error::HasAsyncErrorType;

use crate::traits::{
    CanDecodeMut, CanEncodeMut, HasDecodeBufferType, HasEncodeBufferType, MutDecoder, MutEncoder,
};

pub trait CanEncodeAndDecodeMut<Strategy, Value>:
    CanEncodeMut<Strategy, Value> + CanDecodeMut<Strategy, Value>
{
}

impl<Encoding, Strategy, Value> CanEncodeAndDecodeMut<Strategy, Value> for Encoding where
    Encoding: CanEncodeMut<Strategy, Value> + CanDecodeMut<Strategy, Value>
{
}

pub trait MutEncoderAndDecoder<Encoding, Strategy, Value>:
    MutEncoder<Encoding, Strategy, Value> + MutDecoder<Encoding, Strategy, Value>
where
    Encoding: HasEncodeBufferType + HasDecodeBufferType + HasAsyncErrorType,
{
}

impl<Component, Encoding, Strategy, Value> MutEncoderAndDecoder<Encoding, Strategy, Value>
    for Component
where
    Component: MutEncoder<Encoding, Strategy, Value> + MutDecoder<Encoding, Strategy, Value>,
    Encoding: HasEncodeBufferType + HasDecodeBufferType + HasAsyncErrorType,
{
}
