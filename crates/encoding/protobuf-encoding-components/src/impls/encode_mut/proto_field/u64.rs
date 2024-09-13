use cgp::prelude::HasErrorType;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use prost::bytes::BufMut;
use prost::encoding::{encode_key, encode_varint, WireType};

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

#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct Height {
    /// the revision that the client is currently on
    #[prost(uint64, tag = "1")]
    pub revision_number: u64,
    /// the height within the given revision
    #[prost(uint64, tag = "2")]
    pub revision_height: u64,
}
