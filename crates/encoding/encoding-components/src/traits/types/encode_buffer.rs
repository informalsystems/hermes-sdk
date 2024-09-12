use cgp::prelude::*;

use crate::traits::types::encoded::HasEncodedType;

#[derive_component(EncodeBufferTypeComponent, ProvideEncodeBufferType<Encoding>)]
pub trait HasEncodeBufferType {
    type EncodeBuffer: Default;
}

#[derive_component(EncodeBufferFinalizerComponent, EncodeBufferFinalizer<Encoding>)]
pub trait CanFinalizedEncodeBuffer: HasEncodeBufferType + HasEncodedType {
    fn to_encoded(buffer: Self::EncodeBuffer) -> Self::Encoded;
}
