use cgp::prelude::{CanRaiseError, HasErrorType};
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use prost::bytes::BufMut;
use prost::{DecodeError, Message};

use crate::impls::encode_mut::chunk::{
    CanDecodeProtoChunks, HasProtoChunksDecodeBuffer, ProtoChunks,
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

        for (tag, (wire_type, mut bytes)) in chunks.iter() {
            value
                .merge_field(*tag, *wire_type, &mut bytes, Default::default())
                .map_err(Encoding::raise_error)?;
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
