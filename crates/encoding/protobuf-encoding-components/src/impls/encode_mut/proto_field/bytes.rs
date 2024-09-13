use cgp::prelude::*;
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use prost::bytes::BufMut;

use crate::impls::encode_mut::chunk::{
    HasProtoChunksDecodeBuffer, InvalidWireType, ProtoChunk, ProtoChunks,
};

use prost::encoding::{encode_key, encode_varint, WireType};

pub struct EncodeByteField<const TAG: u32>;

impl<Encoding, Strategy, const TAG: u32> MutEncoder<Encoding, Strategy, Vec<u8>>
    for EncodeByteField<TAG>
where
    Encoding: HasEncodeBufferType<EncodeBuffer: BufMut> + HasErrorType,
{
    fn encode_mut(
        _encoding: &Encoding,
        value: &Vec<u8>,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        if value.len() > 0 {
            encode_key(TAG, WireType::LengthDelimited, buffer);
            encode_varint(value.len() as u64, buffer);

            buffer.put(value.as_ref());
        }

        Ok(())
    }
}

impl<Encoding, Strategy, const TAG: u32> MutDecoder<Encoding, Strategy, Vec<u8>>
    for EncodeByteField<TAG>
where
    Encoding: HasProtoChunksDecodeBuffer + CanRaiseError<InvalidWireType>,
{
    fn decode_mut<'a>(
        _encoding: &Encoding,
        chunks: &mut ProtoChunks<'a>,
    ) -> Result<Vec<u8>, Encoding::Error> {
        match chunks.get(&TAG) {
            Some(chunk) => match chunk {
                ProtoChunk::LengthDelimited(bytes) => Ok(Vec::from(*bytes)),
                _ => Err(Encoding::raise_error(InvalidWireType {
                    expected: WireType::LengthDelimited,
                    actual: chunk.wire_type(),
                })),
            },
            None => Ok(Vec::new()),
        }
    }
}

// impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Vec<u8>> for EncodeBytes
// where
//     Encoding: HasEncodeBufferType<EncodeBuffer: BufMut> + HasErrorType,
// {
//     fn encode_mut(
//         _encoding: &Encoding,
//         value: &Vec<u8>,
//         buffer: &mut Encoding::EncodeBuffer,
//     ) -> Result<(), Encoding::Error> {
//         buffer.put(value.as_ref());

//         Ok(())
//     }
// }

// impl<Encoding, Strategy> EncodedLengthGetter<Encoding, Strategy, Vec<u8>> for EncodeBytes
// {
//     fn encoded_length(_encoding: &Encoding, value: &Vec<u8>) -> u64 {
//         value.len() as u64
//     }
// }
