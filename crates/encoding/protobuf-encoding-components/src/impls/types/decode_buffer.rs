use hermes_encoding_components::traits::types::decode_buffer::ProvideDecodeBufferType;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;

use crate::impls::encode_mut::chunk::{CanDecodeProtoChunks, ProtoChunks};

pub struct ProvideProtoChunksDecodeBuffer;

impl<Encoding> ProvideDecodeBufferType<Encoding> for ProvideProtoChunksDecodeBuffer
where
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanDecodeProtoChunks,
{
    type DecodeBuffer<'a> = ProtoChunks<'a>;
}
