use cgp::prelude::*;
use hermes_encoding_components::traits::{
    EncodeBufferTypeComponent, HasEncodedType, ProvideEncodeBufferType,
};

pub struct ProvideBytesEncodeBuffer;

#[cgp_provider(EncodeBufferTypeComponent)]
impl<Encoding> ProvideEncodeBufferType<Encoding> for ProvideBytesEncodeBuffer
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>,
{
    type EncodeBuffer = Vec<u8>;
}
