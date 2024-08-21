use cgp_core::prelude::*;

use crate::traits::encoded::HasEncodedType;

#[derive_component(EncodeBufferTypeComponent, ProvideEncodeBufferType<Encoding>)]
pub trait HasEncodeBufferType: HasEncodedType {
    type EncodeBuffer: Default;

    fn to_encoded(buffer: Self::EncodeBuffer) -> Self::Encoded;
}

#[derive_component(MutEncoderComponent, MutEncoder<Encoding>)]
pub trait CanEncodeMut<Strategy, Value>: HasEncodeBufferType + HasErrorType {
    fn encode_mut(&self, value: &Value, buffer: &mut Self::EncodeBuffer)
        -> Result<(), Self::Error>;
}
