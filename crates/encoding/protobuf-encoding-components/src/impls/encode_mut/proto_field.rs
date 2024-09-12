use core::marker::PhantomData;

use cgp::prelude::{CanRaiseError, HasErrorType};
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use prost::encoding::{check_wire_type, encode_key, encode_varint, WireType};
use prost::DecodeError;

use crate::impls::encode_mut::chunk::{
    CanDecodeProtoChunks, HasProtoChunksDecodeBuffer, ProtoChunks,
};

pub struct EncodeProtoField<const TAG: u32, InEncoder>(pub PhantomData<InEncoder>);

impl<Encoding, Strategy, Value, InEncoder, const TAG: u32> MutEncoder<Encoding, Strategy, Value>
    for EncodeProtoField<TAG, InEncoder>
where
    Encoding: HasEncodeBufferType<EncodeBuffer = Vec<u8>> + HasErrorType,
    InEncoder: MutEncoder<Encoding, Strategy, Value>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Vec<u8>,
    ) -> Result<(), Encoding::Error> {
        let mut in_buffer = Vec::new();

        InEncoder::encode_mut(encoding, value, &mut in_buffer)?;

        encode_key(TAG, WireType::LengthDelimited, buffer);
        encode_varint(in_buffer.len() as u64, buffer);

        buffer.append(&mut in_buffer);

        Ok(())
    }
}

impl<Encoding, Strategy, Value, InEncoder, const TAG: u32>
    MutDecoder<Encoding, Strategy, Option<Value>> for EncodeProtoField<TAG, InEncoder>
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
