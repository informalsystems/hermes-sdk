use cgp::core::error::HasAsyncErrorType;

use crate::traits::decode_mut::{CanDecodeMut, MutDecoder};
use crate::traits::encode_mut::{CanEncodeMut, MutEncoder};
use crate::traits::types::decode_buffer::HasDecodeBufferType;
use crate::traits::types::encode_buffer::HasEncodeBufferType;

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
