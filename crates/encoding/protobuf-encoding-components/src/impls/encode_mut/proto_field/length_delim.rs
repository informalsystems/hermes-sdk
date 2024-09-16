use cgp::prelude::HasErrorType;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use prost::bytes::BufMut;
use prost::encoding::{encode_key, encode_varint, WireType};

pub struct EncodeLengthDelimitedHeader<const TAG: u32>;

impl<Encoding, Strategy, const TAG: u32> MutEncoder<Encoding, Strategy, u64>
    for EncodeLengthDelimitedHeader<TAG>
where
    Encoding: HasEncodeBufferType<EncodeBuffer: BufMut> + HasErrorType,
{
    fn encode_mut(
        _encoding: &Encoding,
        length: &u64,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        encode_key(TAG, WireType::LengthDelimited, buffer);
        encode_varint(*length, buffer);

        Ok(())
    }
}
