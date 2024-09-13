use cgp::prelude::HasErrorType;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use prost::bytes::BufMut;
use prost::encoding::{encode_key, encode_varint, WireType};

pub struct EncodeU64ProtoField<const TAG: u32>;

impl<Encoding, Strategy, const TAG: u32> MutEncoder<Encoding, Strategy, u64>
    for EncodeU64ProtoField<TAG>
where
    Encoding: HasEncodeBufferType<EncodeBuffer: BufMut> + HasErrorType,
{
    fn encode_mut(
        _encoding: &Encoding,
        value: &u64,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encode_key(TAG, WireType::Varint, buffer);
        encode_varint(*value, buffer);

        Ok(())
    }
}
