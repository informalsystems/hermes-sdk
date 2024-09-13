use cgp::prelude::{CanRaiseError, HasErrorType};
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use prost::bytes::BufMut;
use prost::encoding::{check_wire_type, decode_varint, encode_key, encode_varint, WireType};
use prost::DecodeError;

use crate::impls::encode_mut::chunk::{HasProtoChunksDecodeBuffer, ProtoChunks};

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
    Encoding: HasProtoChunksDecodeBuffer + CanRaiseError<DecodeError> + CanRaiseError<Value::Error>,
    Value: TryFrom<u64>,
{
    fn decode_mut<'a>(
        _encoding: &Encoding,
        chunks: &mut ProtoChunks<'a>,
    ) -> Result<Value, Encoding::Error> {
        let value = match chunks.get(&TAG) {
            Some((wire_type, mut bytes)) => {
                check_wire_type(WireType::Varint, *wire_type).map_err(Encoding::raise_error)?;

                decode_varint(&mut bytes).map_err(Encoding::raise_error)?
            }
            None => 0,
        };

        value.try_into().map_err(Encoding::raise_error)
    }
}
