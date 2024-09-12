use hermes_encoding_components::traits::types::encode_buffer::ProvideEncodeBufferType;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;

pub struct ProvideBytesEncodeBuffer;

impl<Encoding> ProvideEncodeBufferType<Encoding> for ProvideBytesEncodeBuffer
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>,
{
    type EncodeBuffer = Vec<u8>;

    fn to_encoded(buffer: Vec<u8>) -> Vec<u8> {
        buffer
    }
}
