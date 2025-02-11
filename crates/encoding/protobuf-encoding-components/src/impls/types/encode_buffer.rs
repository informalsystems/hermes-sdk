use cgp::prelude::*;
use hermes_encoding_components::traits::types::encode_buffer::{
    EncodeBufferTypeComponent, ProvideEncodeBufferType,
};
use hermes_encoding_components::traits::types::encoded::HasEncodedType;

pub struct ProvideBytesEncodeBuffer;

#[cgp_provider(EncodeBufferTypeComponent)]
impl<Encoding> ProvideEncodeBufferType<Encoding> for ProvideBytesEncodeBuffer
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>,
{
    type EncodeBuffer = Vec<u8>;
}
