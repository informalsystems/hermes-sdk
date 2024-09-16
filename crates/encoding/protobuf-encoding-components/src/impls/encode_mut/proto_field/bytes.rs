use cgp::prelude::*;
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use prost::bytes::BufMut;

use crate::impls::encode_mut::chunk::{HasProtoChunksDecodeBuffer, InvalidWireType, ProtoChunks};
use crate::impls::encode_mut::proto_field::length_delim::EncodeLengthDelimited;

pub struct EncodeByteField<const TAG: u32>;

impl<Encoding, Strategy, Value, const TAG: u32> MutEncoder<Encoding, Strategy, Value>
    for EncodeByteField<TAG>
where
    Encoding: HasEncodeBufferType<EncodeBuffer: BufMut> + HasErrorType,
    EncodeLengthDelimited<TAG>: MutEncoder<Encoding, Strategy, u64>,
    Value: AsRef<[u8]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let bytes = value.as_ref();
        if !bytes.is_empty() {
            <EncodeLengthDelimited<TAG>>::encode_mut(encoding, &(bytes.len() as u64), buffer)?;
            buffer.put(bytes);
        }

        Ok(())
    }
}

impl<Encoding, Strategy, Value, const TAG: u32> MutDecoder<Encoding, Strategy, Value>
    for EncodeByteField<TAG>
where
    Encoding: HasProtoChunksDecodeBuffer + CanRaiseError<InvalidWireType>,
    Value: Default + for<'a> From<&'a [u8]>,
{
    fn decode_mut(
        _encoding: &Encoding,
        chunks: &mut ProtoChunks<'_>,
    ) -> Result<Value, Encoding::Error> {
        match chunks.get(&TAG) {
            Some(chunk) => {
                let bytes = chunk.to_length_delimited().map_err(Encoding::raise_error)?;
                Ok(Value::from(bytes))
            }
            None => Ok(Value::default()),
        }
    }
}
