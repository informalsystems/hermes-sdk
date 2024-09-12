use cgp::prelude::CanRaiseError;
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use prost::{DecodeError, Message};

use crate::impls::encode_mut::chunk::{
    CanDecodeProtoChunks, HasProtoChunksDecodeBuffer, ProtoChunks,
};

pub struct EncodeProstMessage;

impl<Encoding, Strategy, Value> MutDecoder<Encoding, Strategy, Value> for EncodeProstMessage
where
    Value: Message + Default,
    Encoding: CanDecodeProtoChunks + HasProtoChunksDecodeBuffer + CanRaiseError<DecodeError>,
{
    fn decode_mut(
        _encoding: &Encoding,
        chunks: &mut ProtoChunks<'_>,
    ) -> Result<Value, Encoding::Error> {
        let mut value = Value::default();

        for (tag, (wire_type, mut bytes)) in chunks.iter() {
            value
                .merge_field(*tag, *wire_type, &mut bytes, Default::default())
                .map_err(Encoding::raise_error)?;
        }

        Ok(value)
    }
}
