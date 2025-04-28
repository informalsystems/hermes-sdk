use hermes_prelude::*;

use crate::traits::HasEncodedType;

#[cgp_component {
  name: DecodeBufferTypeComponent,
  provider: ProvideDecodeBufferType,
  context: Encoding,
}]
pub trait HasDecodeBufferType {
    type DecodeBuffer<'a>;
}

#[cgp_component {
  provider: DecodeBufferBuilder,
  context: Encoding,
}]
pub trait CanBuildDecodeBuffer: HasDecodeBufferType + HasEncodedType {
    fn from_encoded<'a>(encoded: &'a Self::Encoded) -> Self::DecodeBuffer<'a>;
}
