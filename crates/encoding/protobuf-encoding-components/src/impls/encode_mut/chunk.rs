use std::collections::BTreeMap;

use cgp::prelude::{CanRaiseError, HasErrorType};
use hermes_encoding_components::traits::types::decode_buffer::HasDecodeBufferType;
use prost::bytes::Buf;
use prost::encoding::{decode_key, decode_varint, WireType};
use prost::DecodeError;

pub struct ProtoChunk<'a> {
    pub tag: u32,
    pub wire_type: WireType,
    pub chunk: &'a [u8],
}

pub type ProtoChunks<'a> = BTreeMap<u32, (WireType, &'a [u8])>;

#[derive(Debug)]
pub struct UnsupportedWireType {
    pub wire_type: WireType,
}

pub trait CanDecodeProtoChunks: HasErrorType {
    fn decode_protochunks<'a>(buffer: &mut &'a [u8]) -> Result<ProtoChunks<'a>, Self::Error>;
}

pub trait CanDecodeProtoChunk: HasErrorType {
    fn decode_protochunk<'a>(buffer: &mut &'a [u8]) -> Result<ProtoChunk<'a>, Self::Error>;
}

impl<Encoding> CanDecodeProtoChunks for Encoding
where
    Encoding: CanDecodeProtoChunk,
{
    fn decode_protochunks<'a>(buffer: &mut &'a [u8]) -> Result<ProtoChunks<'a>, Self::Error> {
        let mut chunks = BTreeMap::new();

        while buffer.len() > 0 {
            let chunk = Self::decode_protochunk(buffer)?;
            chunks.insert(chunk.tag, (chunk.wire_type, chunk.chunk));
        }

        Ok(chunks)
    }
}

impl<Encoding> CanDecodeProtoChunk for Encoding
where
    Encoding: CanRaiseError<DecodeError> + CanRaiseError<UnsupportedWireType>,
{
    fn decode_protochunk<'a>(buffer: &mut &'a [u8]) -> Result<ProtoChunk<'a>, Encoding::Error> {
        let (tag, wire_type) = decode_key(buffer).map_err(Encoding::raise_error)?;

        let length = match wire_type {
            WireType::Varint => decode_varint(buffer)
                .map(|_| 0)
                .map_err(Encoding::raise_error)?,
            WireType::ThirtyTwoBit => 4,
            WireType::SixtyFourBit => 8,
            WireType::LengthDelimited => decode_varint(buffer).map_err(Encoding::raise_error)?,
            _ => return Err(Encoding::raise_error(UnsupportedWireType { wire_type })),
        } as usize;

        if length > buffer.len() {
            return Err(Encoding::raise_error(DecodeError::new("buffer underflow")));
        }

        let chunk = &buffer[..length];

        buffer.advance(length);

        Ok(ProtoChunk {
            tag,
            wire_type,
            chunk,
        })
    }
}

pub trait HasProtoChunksDecodeBuffer:
    for<'a> HasDecodeBufferType<DecodeBuffer<'a> = ProtoChunks<'a>>
{
}

impl<Encoding> HasProtoChunksDecodeBuffer for Encoding where
    Encoding: for<'a> HasDecodeBufferType<DecodeBuffer<'a> = ProtoChunks<'a>>
{
}
