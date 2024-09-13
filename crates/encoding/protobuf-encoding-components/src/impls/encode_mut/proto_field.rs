use core::marker::PhantomData;

use cgp::prelude::HasErrorType;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use prost::encoding::{encode_key, encode_varint, WireType};

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
