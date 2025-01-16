use core::array::TryFromSliceError;
use std::collections::BTreeMap;

use cgp::prelude::{CanRaiseAsyncError, HasAsyncErrorType};
use hermes_encoding_components::traits::types::decode_buffer::HasDecodeBufferType;
use prost::bytes::Buf;
use prost::encoding::{decode_key, decode_varint, WireType};
use prost::DecodeError;

#[derive(Debug)]
pub enum ProtoChunk<'a> {
    Varint(u64),
    LengthDelimited(&'a [u8]),
    ThirtyTwoBit([u8; 4]),
    SixtyFourBit([u8; 8]),
}

pub type ProtoChunks<'a> = BTreeMap<u32, ProtoChunk<'a>>;

#[derive(Debug)]
pub struct UnsupportedWireType {
    pub wire_type: WireType,
}

#[derive(Debug)]
pub struct InvalidWireType {
    pub expected: WireType,
    pub actual: WireType,
}

pub trait CanDecodeProtoChunks: HasAsyncErrorType {
    fn decode_protochunks<'a>(buffer: &mut &'a [u8]) -> Result<ProtoChunks<'a>, Self::Error>;
}

pub trait CanDecodeProtoChunk: HasAsyncErrorType {
    fn decode_protochunk<'a>(buffer: &mut &'a [u8]) -> Result<(u32, ProtoChunk<'a>), Self::Error>;
}

impl<'a> ProtoChunk<'a> {
    pub fn wire_type(&self) -> WireType {
        match self {
            Self::Varint(_) => WireType::Varint,
            Self::LengthDelimited(_) => WireType::LengthDelimited,
            Self::ThirtyTwoBit(_) => WireType::ThirtyTwoBit,
            Self::SixtyFourBit(_) => WireType::SixtyFourBit,
        }
    }

    pub fn to_length_delimited(&self) -> Result<&'a [u8], InvalidWireType> {
        match self {
            Self::LengthDelimited(bytes) => Ok(bytes),
            _ => Err(InvalidWireType {
                expected: WireType::LengthDelimited,
                actual: self.wire_type(),
            }),
        }
    }

    pub fn to_varint(&self) -> Result<u64, InvalidWireType> {
        match self {
            Self::Varint(value) => Ok(*value),
            _ => Err(InvalidWireType {
                expected: WireType::Varint,
                actual: self.wire_type(),
            }),
        }
    }
}

impl<Encoding> CanDecodeProtoChunks for Encoding
where
    Encoding: CanDecodeProtoChunk,
{
    fn decode_protochunks<'a>(buffer: &mut &'a [u8]) -> Result<ProtoChunks<'a>, Self::Error> {
        let mut chunks = BTreeMap::new();

        while !buffer.is_empty() {
            let (tag, chunk) = Self::decode_protochunk(buffer)?;
            chunks.insert(tag, chunk);
        }

        Ok(chunks)
    }
}

impl<Encoding> CanDecodeProtoChunk for Encoding
where
    Encoding: CanRaiseAsyncError<DecodeError>
        + CanRaiseAsyncError<UnsupportedWireType>
        + CanRaiseAsyncError<TryFromSliceError>,
{
    fn decode_protochunk<'a>(
        buffer: &mut &'a [u8],
    ) -> Result<(u32, ProtoChunk<'a>), Encoding::Error> {
        let (tag, wire_type) = decode_key(buffer).map_err(Encoding::raise_error)?;

        let chunk = match wire_type {
            WireType::Varint => {
                let value = decode_varint(buffer).map_err(Encoding::raise_error)?;

                ProtoChunk::Varint(value)
            }
            WireType::LengthDelimited => {
                let length = decode_varint(buffer).map_err(Encoding::raise_error)? as usize;
                let bytes = &buffer[..length];
                buffer.advance(length);

                ProtoChunk::LengthDelimited(bytes)
            }
            WireType::ThirtyTwoBit => {
                let bytes = <[u8; 4]>::try_from(&buffer[..4]).map_err(Encoding::raise_error)?;

                ProtoChunk::ThirtyTwoBit(bytes)
            }
            WireType::SixtyFourBit => {
                let bytes = <[u8; 8]>::try_from(&buffer[..8]).map_err(Encoding::raise_error)?;

                ProtoChunk::SixtyFourBit(bytes)
            }
            _ => return Err(Encoding::raise_error(UnsupportedWireType { wire_type })),
        };

        Ok((tag, chunk))
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
