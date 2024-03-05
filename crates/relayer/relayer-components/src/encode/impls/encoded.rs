use alloc::vec::Vec;
use cgp_core::prelude::Async;

use crate::encode::traits::encoded::ProvideEncodedType;

pub struct ProvideEncodedBytesType;

impl<Encode> ProvideEncodedType<Encode> for ProvideEncodedBytesType
where
    Encode: Async,
{
    type Encoded = Vec<u8>;
}
