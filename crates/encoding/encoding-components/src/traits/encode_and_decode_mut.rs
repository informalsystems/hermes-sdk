use cgp_core::error::HasErrorType;

use crate::traits::decode_mut::{CanDecodeMut, HasDecodeBufferType, MutDecoder};
use crate::traits::encode_mut::{CanEncodeMut, HasEncodeBufferType, MutEncoder};

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
    Encoding: HasEncodeBufferType + HasDecodeBufferType + HasErrorType,
{
}

impl<Component, Encoding, Strategy, Value> MutEncoderAndDecoder<Encoding, Strategy, Value>
    for Component
where
    Component: MutEncoder<Encoding, Strategy, Value> + MutDecoder<Encoding, Strategy, Value>,
    Encoding: HasEncodeBufferType + HasDecodeBufferType + HasErrorType,
{
}
