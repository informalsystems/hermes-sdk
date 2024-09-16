use cgp::prelude::*;

use crate::traits::types::encoded::HasEncodedType;

#[derive_component(DecodeBufferTypeComponent, ProvideDecodeBufferType<Encoding>)]
pub trait HasDecodeBufferType {
    type DecodeBuffer<'a>;
}

#[derive_component(DecodeBufferBuilderComponent, DecodeBufferBuilder<Encoding>)]
pub trait CanBuildDecodeBuffer: HasDecodeBufferType + HasEncodedType {
    fn from_encoded<'a>(encoded: &'a Self::Encoded) -> Self::DecodeBuffer<'a>;
}
