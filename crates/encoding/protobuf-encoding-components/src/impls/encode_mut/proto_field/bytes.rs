use cgp::prelude::*;
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use prost::bytes::BufMut;

use crate::impls::encode_mut::chunk::{HasProtoChunksDecodeBuffer, InvalidWireType, ProtoChunks};
use crate::impls::encode_mut::proto_field::length_delim::EncodeLengthDelimited;

pub struct EncodeByteField<const TAG: u32>;

impl<Encoding, Strategy, const TAG: u32> MutEncoder<Encoding, Strategy, Vec<u8>>
    for EncodeByteField<TAG>
where
    Encoding: HasEncodeBufferType<EncodeBuffer: BufMut> + HasErrorType,
    EncodeLengthDelimited<TAG>: MutEncoder<Encoding, Strategy, u64>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Vec<u8>,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        if value.len() > 0 {
            <EncodeLengthDelimited<TAG>>::encode_mut(encoding, &(value.len() as u64), buffer)?;
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
            Some(chunk) => {
                let bytes = chunk.to_length_delimited().map_err(Encoding::raise_error)?;
                Ok(Vec::from(bytes))
            }
            None => Ok(Vec::new()),
        }
    }
}
