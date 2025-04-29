use hermes_encoding_components::traits::{
    DecodeBufferTypeComponent, HasEncodedType, ProvideDecodeBufferType,
};
use hermes_prelude::*;

use crate::impls::{CanDecodeProtoChunks, ProtoChunks};

pub struct ProvideProtoChunksDecodeBuffer;

#[cgp_provider(DecodeBufferTypeComponent)]
impl<Encoding> ProvideDecodeBufferType<Encoding> for ProvideProtoChunksDecodeBuffer
where
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanDecodeProtoChunks,
{
    type DecodeBuffer<'a> = ProtoChunks<'a>;
}
