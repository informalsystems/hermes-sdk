use alloc::vec::Vec;
use cgp_core::prelude::*;

#[derive_component(EncodedTypeComponent, ProvideEncodedType<Encoding>)]
pub trait HasEncodedType: Async {
    type Encoded: Async;
}

pub struct ProvideEncodedBytesType;

impl<Encode> ProvideEncodedType<Encode> for ProvideEncodedBytesType
where
    Encode: Async,
{
    type Encoded = Vec<u8>;
}
