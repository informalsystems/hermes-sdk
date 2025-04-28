use hermes_prelude::*;

use crate::traits::HasEncodedType;

#[cgp_component {
  name: EncodeBufferTypeComponent,
  provider: ProvideEncodeBufferType,
  context: Encoding,
}]
pub trait HasEncodeBufferType {
    type EncodeBuffer: Default;
}

#[cgp_component {
  provider: EncodeBufferFinalizer,
  context: Encoding,
}]
pub trait CanFinalizedEncodeBuffer: HasEncodeBufferType + HasEncodedType {
    fn to_encoded(buffer: Self::EncodeBuffer) -> Self::Encoded;
}
