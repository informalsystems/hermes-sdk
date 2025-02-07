use cgp::prelude::*;
use hermes_encoding_components::traits::types::decode_buffer::{
    DecodeBufferTypeComponent, ProvideDecodeBufferType,
};
use hermes_encoding_components::traits::types::encoded::HasEncodedType;

use crate::impls::encode_mut::chunk::{CanDecodeProtoChunks, ProtoChunks};

pub struct ProvideProtoChunksDecodeBuffer;

#[cgp_provider(DecodeBufferTypeComponent)]
impl<Encoding> ProvideDecodeBufferType<Encoding> for ProvideProtoChunksDecodeBuffer
where
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanDecodeProtoChunks,
{
    type DecodeBuffer<'a> = ProtoChunks<'a>;
}
