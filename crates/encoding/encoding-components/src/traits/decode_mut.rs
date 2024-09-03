use cgp::prelude::*;

use crate::traits::types::decode_buffer::HasDecodeBufferType;

#[derive_component(MutDecoderComponent, MutDecoder<Encoding>)]
pub trait CanDecodeMut<Strategy, Value>: HasDecodeBufferType + HasErrorType {
    fn decode_mut(&self, buffer: &mut Self::DecodeBuffer<'_>) -> Result<Value, Self::Error>;
}

#[derive_component(DecodeBufferPeekerComponent, DecodeBufferPeeker<Encoding>)]
pub trait CanPeekDecodeBuffer<Value>: HasDecodeBufferType {
    fn peek_decode_buffer<'a>(buffer: &'a mut Self::DecodeBuffer<'_>) -> Option<&'a Value>;
}
