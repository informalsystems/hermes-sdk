use cgp::prelude::CanRaiseError;
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use prost::bytes::BufMut;
use prost::encoding::{encode_key, encode_varint, WireType};

use crate::impls::encode_mut::chunk::{HasProtoChunksDecodeBuffer, InvalidWireType, ProtoChunks};

pub struct EncodeU64ProtoField<const TAG: u32>;

impl<Encoding, Strategy, Value, const TAG: u32> MutEncoder<Encoding, Strategy, Value>
    for EncodeU64ProtoField<TAG>
where
    Encoding: HasEncodeBufferType<EncodeBuffer: BufMut> + CanRaiseError<Value::Error>,
    Value: Clone + TryInto<u64>,
{
    fn encode_mut(
        _encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let value2 = value.clone().try_into().map_err(Encoding::raise_error)?;

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
    fn decode_mut(
        _encoding: &Encoding,
        chunks: &mut ProtoChunks<'_>,
    ) -> Result<Value, Encoding::Error> {
        let value = match chunks.get(&TAG) {
            Some(chunk) => chunk.to_varint().map_err(Encoding::raise_error)?,
            None => 0,
        };

        value.try_into().map_err(Encoding::raise_error)
    }
}
