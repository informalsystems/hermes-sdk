use cgp::prelude::{CanRaiseError, HasErrorType};
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use prost::bytes::BufMut;
use prost::encoding::{encode_key, encode_varint, WireType};

use crate::impls::encode_mut::chunk::{
    HasProtoChunksDecodeBuffer, InvalidWireType, ProtoChunk, ProtoChunks,
};

pub struct EncodeU64ProtoField<const TAG: u32>;

impl<Encoding, Strategy, Value, const TAG: u32> MutEncoder<Encoding, Strategy, Value>
    for EncodeU64ProtoField<TAG>
where
    Encoding: HasEncodeBufferType<EncodeBuffer: BufMut> + HasErrorType,
    Value: Clone + Into<u64>,
{
    fn encode_mut(
        _encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let value2 = value.clone().into();

        if value2 != 0 {
            encode_key(TAG, WireType::Varint, buffer);
            encode_varint(value2, buffer);
        }

        Ok(())
    }
}

impl<Encoding, Strategy, Value, const TAG: u32> MutDecoder<Encoding, Strategy, Value>
    for EncodeU64ProtoField<TAG>
where
    Encoding:
        HasProtoChunksDecodeBuffer + CanRaiseError<InvalidWireType> + CanRaiseError<Value::Error>,
    Value: TryFrom<u64>,
{
    fn decode_mut<'a>(
        _encoding: &Encoding,
        chunks: &mut ProtoChunks<'a>,
    ) -> Result<Value, Encoding::Error> {
        let value = match chunks.get(&TAG) {
            Some(chunk) => match chunk {
                ProtoChunk::Varint(value) => *value,
                _ => {
                    return Err(Encoding::raise_error(InvalidWireType {
                        expected: WireType::Varint,
                        actual: chunk.wire_type(),
                    }))
                }
            },
            None => 0,
        };

        value.try_into().map_err(Encoding::raise_error)
    }
}
