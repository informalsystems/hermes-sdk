use cgp::prelude::HasErrorType;
use hermes_encoding_components::traits::decode::Decoder;
use hermes_encoding_components::traits::decode_mut::CanDecodeMut;
use hermes_encoding_components::traits::encode::Encoder;
use hermes_encoding_components::traits::encode_mut::CanEncodeMut;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;

use crate::impls::encode_mut::chunk::{CanDecodeProtoChunks, HasProtoChunksDecodeBuffer};

pub struct EncodeProtoWithMutBuffer;

impl<Encoding, Strategy, Value> Encoder<Encoding, Strategy, Value> for EncodeProtoWithMutBuffer
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanEncodeMut<Strategy, Value>
        + HasEncodeBufferType<EncodeBuffer = Vec<u8>>,
{
    fn encode(encoding: &Encoding, value: &Value) -> Result<Vec<u8>, Encoding::Error> {
        let mut buffer = Vec::new();

        encoding.encode_mut(value, &mut buffer)?;

        Ok(buffer)
    }
}

impl<Encoding, Strategy, Value> Decoder<Encoding, Strategy, Value> for EncodeProtoWithMutBuffer
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + CanDecodeProtoChunks
        + HasProtoChunksDecodeBuffer
        + CanDecodeMut<Strategy, Value>,
{
    fn decode(
        encoding: &Encoding,
        buffer: &Vec<u8>,
    ) -> Result<Value, <Encoding as HasErrorType>::Error> {
        let mut chunks = Encoding::decode_protochunks(&mut buffer.as_ref())?;

        encoding.decode_mut(&mut chunks)
    }
}
