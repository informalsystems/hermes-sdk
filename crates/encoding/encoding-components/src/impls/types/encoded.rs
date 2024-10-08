use alloc::vec::Vec;

use cgp::prelude::Async;

use crate::traits::types::encoded::ProvideEncodedType;

pub struct ProvideEncodedBytes;

impl<Encode> ProvideEncodedType<Encode> for ProvideEncodedBytes
where
    Encode: Async,
{
    type Encoded = Vec<u8>;
}
