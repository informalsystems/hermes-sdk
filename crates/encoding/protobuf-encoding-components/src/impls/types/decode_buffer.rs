use cgp::prelude::*;
use hermes_encoding_components::traits::{
    DecodeBufferTypeComponent, HasEncodedType, ProvideDecodeBufferType,
};

use crate::impls::encode_mut::chunk::{CanDecodeProtoChunks, ProtoChunks};

pub struct ProvideProtoChunksDecodeBuffer;

#[cgp_provider(DecodeBufferTypeComponent)]
impl<Encoding> ProvideDecodeBufferType<Encoding> for ProvideProtoChunksDecodeBuffer
where
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanDecodeProtoChunks,
{
    type DecodeBuffer<'a> = ProtoChunks<'a>;
}
