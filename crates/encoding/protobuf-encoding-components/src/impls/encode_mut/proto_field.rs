use core::marker::PhantomData;

use cgp::prelude::{CanRaiseError, HasErrorType};
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use prost::bytes::BufMut;
use prost::encoding::{
    check_wire_type, encode_key, encode_varint, encoded_len_varint, key_len, WireType,
};
use prost::DecodeError;

use crate::impls::encode_mut::chunk::{
    CanDecodeProtoChunks, HasProtoChunksDecodeBuffer, ProtoChunks,
};
use crate::traits::length::EncodedLengthGetter;

pub struct EncodeProtoField<InEncoder, const TAG: u32>(pub PhantomData<InEncoder>);

impl<Encoding, Strategy, Value, InEncoder, const TAG: u32> MutEncoder<Encoding, Strategy, Value>
    for EncodeProtoField<InEncoder, TAG>
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
    MutDecoder<Encoding, Strategy, Option<Value>> for EncodeProtoField<InEncoder, TAG>
where
    Encoding: CanDecodeProtoChunks + HasProtoChunksDecodeBuffer + CanRaiseError<DecodeError>,
    InEncoder: MutDecoder<Encoding, Strategy, Value>,
{
    fn decode_mut(
        encoding: &Encoding,
        chunks: &mut ProtoChunks<'_>,
    ) -> Result<Option<Value>, Encoding::Error> {
        if let Some((wire_type, mut bytes)) = chunks.get(&TAG) {
            check_wire_type(WireType::LengthDelimited, *wire_type)
                .map_err(Encoding::raise_error)?;

            let mut in_chunks = Encoding::decode_protochunks(&mut bytes)?;

            let value = InEncoder::decode_mut(encoding, &mut in_chunks)?;

            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

impl<Encoding, Strategy, Value, InEncoder, const TAG: u32>
    EncodedLengthGetter<Encoding, Strategy, Value> for EncodeProtoField<InEncoder, TAG>
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
