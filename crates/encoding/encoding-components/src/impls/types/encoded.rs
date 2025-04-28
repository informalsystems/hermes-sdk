use alloc::vec::Vec;

use hermes_prelude::*;

use crate::traits::{EncodedTypeComponent, ProvideEncodedType};

pub struct ProvideEncodedBytes;

#[cgp_provider(EncodedTypeComponent)]
impl<Encode> ProvideEncodedType<Encode> for ProvideEncodedBytes
where
    Encode: Async,
{
    type Encoded = Vec<u8>;
}
