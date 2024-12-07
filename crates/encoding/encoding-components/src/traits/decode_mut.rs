use cgp::prelude::*;

use crate::traits::types::decode_buffer::HasDecodeBufferType;

#[cgp_component {
  name: MutDecoderComponent,
  provider: MutDecoder,
  context: Encoding,
}]
pub trait CanDecodeMut<Strategy, Value>: HasDecodeBufferType + HasErrorType {
    fn decode_mut<'a>(&self, buffer: &mut Self::DecodeBuffer<'a>) -> Result<Value, Self::Error>;
}

#[cgp_component {
  name: DecodeBufferPeekerComponent,
  provider: DecodeBufferPeeker,
  context: Encoding,
}]
pub trait CanPeekDecodeBuffer<Value>: HasDecodeBufferType {
    fn peek_decode_buffer<'a>(buffer: &'a mut Self::DecodeBuffer<'_>) -> Option<&'a Value>;
}
