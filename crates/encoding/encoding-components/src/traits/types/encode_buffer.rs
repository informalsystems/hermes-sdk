use cgp::prelude::*;

use crate::traits::types::encoded::HasEncodedType;

#[cgp_component {
  name: EncodeBufferTypeComponent,
  provider: ProvideEncodeBufferType,
  context: Encoding,
}]
pub trait HasEncodeBufferType {
    type EncodeBuffer: Default;
}

#[cgp_component {
  name: EncodeBufferFinalizerComponent,
  provider: EncodeBufferFinalizer,
  context: Encoding,
}]
pub trait CanFinalizedEncodeBuffer: HasEncodeBufferType + HasEncodedType {
    fn to_encoded(buffer: Self::EncodeBuffer) -> Self::Encoded;
}
