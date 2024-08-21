use cgp_core::prelude::*;

use crate::traits::types::encoded::HasEncodedType;

#[derive_component(DecodeBufferTypeComponent, ProvideDecodeBufferType<Encoding>)]
pub trait HasDecodeBufferType: HasEncodedType {
    type DecodeBuffer<'a>;

    fn from_encoded<'a>(encoded: &'a Self::Encoded) -> Self::DecodeBuffer<'a>;
}
