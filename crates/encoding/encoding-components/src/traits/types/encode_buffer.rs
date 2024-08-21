use cgp_core::prelude::*;

use crate::traits::types::encoded::HasEncodedType;

#[derive_component(EncodeBufferTypeComponent, ProvideEncodeBufferType<Encoding>)]
pub trait HasEncodeBufferType: HasEncodedType {
    type EncodeBuffer: Default;

    fn to_encoded(buffer: Self::EncodeBuffer) -> Self::Encoded;
}
