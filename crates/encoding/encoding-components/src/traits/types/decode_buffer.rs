use cgp::prelude::*;

use crate::traits::types::encoded::HasEncodedType;

#[cgp_component {
  name: DecodeBufferTypeComponent,
  provider: ProvideDecodeBufferType,
  context: Encoding,
}]
pub trait HasDecodeBufferType {
    type DecodeBuffer<'a>;
}

#[cgp_component {
  name: DecodeBufferBuilderComponent,
  provider: DecodeBufferBuilder,
  context: Encoding,
}]
pub trait CanBuildDecodeBuffer: HasDecodeBufferType + HasEncodedType {
    fn from_encoded<'a>(encoded: &'a Self::Encoded) -> Self::DecodeBuffer<'a>;
}
