use cgp::prelude::{CanRaiseError, HasErrorType};
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use prost::bytes::BufMut;
use prost::encoding::{encode_varint, WireType};
use prost::{DecodeError, Message};

use crate::impls::encode_mut::chunk::{
    CanDecodeProtoChunks, HasProtoChunksDecodeBuffer, ProtoChunk, ProtoChunks,
};
use crate::traits::length::EncodedLengthGetter;

pub struct EncodeProstMessage;

impl<Encoding, Strategy, Value> MutEncoder<Encoding, Strategy, Value> for EncodeProstMessage
where
    Value: Message,
    Encoding: HasEncodeBufferType<EncodeBuffer: BufMut> + HasErrorType,
{
    fn encode_mut(
        _encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        value.encode_raw(buffer);

        Ok(())
    }
}

impl<Encoding, Strategy, Value> MutDecoder<Encoding, Strategy, Value> for EncodeProstMessage
where
    Value: Message + Default,
    Encoding: CanDecodeProtoChunks + HasProtoChunksDecodeBuffer + CanRaiseError<DecodeError>,
{
    fn decode_mut(
        _encoding: &Encoding,
        chunks: &mut ProtoChunks<'_>,
    ) -> Result<Value, Encoding::Error> {
        let mut value = Value::default();

        for (tag, chunk) in chunks.iter() {
            let context = Default::default();
            match chunk {
                ProtoChunk::Varint(i) => {
                    // Due to the design in prost::Message, we have no choice
                    // but to re-encode a VarInt back into bytes. It was required
                    // to parse the VarInt bytes to properly decode a chunk size,
                    // so we didn't really do unnecessary conversion before.
                    let mut bytes = Vec::new();
                    encode_varint(*i, &mut bytes);

                    value
                        .merge_field(
                            *tag,
                            WireType::LengthDelimited,
                            &mut bytes.as_ref(),
                            context,
                        )
                        .map_err(Encoding::raise_error)?;
                }
                ProtoChunk::LengthDelimited(mut bytes) => {
                    value
                        .merge_field(*tag, WireType::LengthDelimited, &mut bytes, context)
                        .map_err(Encoding::raise_error)?;
                }
                ProtoChunk::ThirtyTwoBit(bytes) => {
                    value
                        .merge_field(*tag, WireType::ThirtyTwoBit, &mut bytes.as_slice(), context)
                        .map_err(Encoding::raise_error)?;
                }
                ProtoChunk::SixtyFourBit(bytes) => {
                    value
                        .merge_field(*tag, WireType::SixtyFourBit, &mut bytes.as_slice(), context)
                        .map_err(Encoding::raise_error)?;
                }
            }
        }

        Ok(value)
    }
}

impl<Encoding, Strategy, Value> EncodedLengthGetter<Encoding, Strategy, Value>
    for EncodeProstMessage
where
    Value: Message,
{
    fn encoded_length(_encoding: &Encoding, value: &Value) -> u64 {
        value.encoded_len() as u64
    }
}
