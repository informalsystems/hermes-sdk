use cgp::prelude::*;

use crate::traits::types::encode_buffer::HasEncodeBufferType;

#[derive_component(MutEncoderComponent, MutEncoder<Encoding>)]
pub trait CanEncodeMut<Strategy, Value>: HasEncodeBufferType + HasErrorType {
    fn encode_mut(&self, value: &Value, buffer: &mut Self::EncodeBuffer)
        -> Result<(), Self::Error>;
}
