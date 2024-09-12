use cgp::prelude::{CanRaiseError, HasErrorType};
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use prost::bytes::BufMut;
use prost::encoding::message::{encode, encoded_len};
use prost::encoding::{check_wire_type, WireType};
use prost::{DecodeError, Message};

use crate::impls::encode_mut::chunk::{
    CanDecodeProtoChunks, HasProtoChunksDecodeBuffer, ProtoChunks,
};
use crate::impls::encode_mut::message::EncodeProstMessage;
use crate::traits::length::EncodedLengthGetter;

pub struct EncodeProstMessageField<const TAG: u32>;

impl<Encoding, Strategy, Value, const TAG: u32> MutEncoder<Encoding, Strategy, Value>
    for EncodeProstMessageField<TAG>
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
    for EncodeProstMessageField<TAG>
where
    Value: Message,
    Encoding: CanDecodeProtoChunks + HasProtoChunksDecodeBuffer + CanRaiseError<DecodeError>,
    EncodeProstMessage: MutDecoder<Encoding, Strategy, Value>,
{
    fn decode_mut(
        encoding: &Encoding,
        chunks: &mut ProtoChunks<'_>,
    ) -> Result<Option<Value>, Encoding::Error> {
        if let Some((wire_type, mut bytes)) = chunks.get(&TAG) {
            check_wire_type(WireType::LengthDelimited, *wire_type)
                .map_err(Encoding::raise_error)?;

            let mut in_chunks = encoding.decode_protochunks(&mut bytes)?;

            let value = EncodeProstMessage::decode_mut(encoding, &mut in_chunks)?;

            Ok(Some(value))
        } else {
            Ok(None)
        }
    }
}

impl<Encoding, Strategy, Value, const TAG: u32> EncodedLengthGetter<Encoding, Strategy, Value>
    for EncodeProstMessageField<TAG>
where
    Value: Message,
{
    fn encoded_length(_encoding: &Encoding, value: &Value) -> u64 {
        encoded_len(TAG, value) as u64
    }
}
