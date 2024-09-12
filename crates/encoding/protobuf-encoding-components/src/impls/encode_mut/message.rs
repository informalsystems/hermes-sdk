use cgp::prelude::HasErrorType;
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::decode_buffer::HasDecodeBufferType;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use prost::bytes::BufMut;
use prost::encoding::message::{encode, encoded_len};
use prost::Message;

use crate::traits::length::EncodedLengthGetter;

pub struct EncodeMutMessageField<const TAG: u32>;

impl<Encoding, Strategy, Value, const TAG: u32> MutEncoder<Encoding, Strategy, Value>
    for EncodeMutMessageField<TAG>
where
    Value: Message,
    Encoding: HasEncodeBufferType + HasErrorType,
    Encoding::EncodeBuffer: BufMut,
{
    fn encode_mut(
        _encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encode(TAG, value, buffer);

        Ok(())
    }
}

impl<Encoding, Strategy, Value, const TAG: u32> MutDecoder<Encoding, Strategy, Option<Value>>
    for EncodeMutMessageField<TAG>
where
    Value: Message,
    Encoding: HasDecodeBufferType + HasErrorType,
{
    fn decode_mut(
        _encoding: &Encoding,
        _buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Option<Value>, Encoding::Error> {
        todo!()
    }
}

impl<Encoding, Strategy, Value, const TAG: u32> EncodedLengthGetter<Encoding, Strategy, Value>
    for EncodeMutMessageField<TAG>
where
    Value: Message,
{
    fn encoded_length(_encoding: &Encoding, value: &Value) -> u64 {
        encoded_len(TAG, value) as u64
    }
}
