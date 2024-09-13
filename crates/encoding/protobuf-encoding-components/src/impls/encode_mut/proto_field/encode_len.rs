use core::marker::PhantomData;

use cgp::prelude::HasErrorType;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use prost::bytes::BufMut;
use prost::encoding::{encode_key, encode_varint, encoded_len_varint, key_len, WireType};

use crate::traits::length::EncodedLengthGetter;

pub struct EncodeProtoFieldWithKnownLength<const TAG: u32, InEncoder>(pub PhantomData<InEncoder>);

impl<Encoding, Strategy, Value, InEncoder, const TAG: u32> MutEncoder<Encoding, Strategy, Value>
    for EncodeProtoFieldWithKnownLength<TAG, InEncoder>
where
    Encoding: HasEncodeBufferType<EncodeBuffer: BufMut> + HasErrorType,
    Encoding::EncodeBuffer: BufMut,
    InEncoder:
        MutEncoder<Encoding, Strategy, Value> + EncodedLengthGetter<Encoding, Strategy, Value>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encode_key(TAG, WireType::LengthDelimited, buffer);
        encode_varint(InEncoder::encoded_length(encoding, value), buffer);

        InEncoder::encode_mut(encoding, value, buffer)?;

        Ok(())
    }
}

impl<Encoding, Strategy, Value, InEncoder, const TAG: u32>
    EncodedLengthGetter<Encoding, Strategy, Value>
    for EncodeProtoFieldWithKnownLength<TAG, InEncoder>
where
    InEncoder: EncodedLengthGetter<Encoding, Strategy, Value>,
{
    fn encoded_length(encoding: &Encoding, value: &Value) -> u64 {
        let field_length = InEncoder::encoded_length(encoding, value);
        let key_length = key_len(TAG) as u64;
        let length_length = encoded_len_varint(field_length) as u64;

        field_length + key_length + length_length
    }
}
