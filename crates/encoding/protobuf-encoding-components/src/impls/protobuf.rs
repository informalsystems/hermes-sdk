use hermes_encoding_components::traits::{
    Decoder, DecoderComponent, Encoder, EncoderComponent, HasEncodedType,
};
use hermes_prelude::*;
use prost::{DecodeError, Message};

pub struct EncodeAsProtobuf;

#[cgp_provider(EncoderComponent)]
impl<Encoding, Strategy, Value> Encoder<Encoding, Strategy, Value> for EncodeAsProtobuf
where
    Value: Message,
    Encoding: HasEncodedType<Encoded = Vec<u8>> + HasAsyncErrorType,
{
    fn encode(_encoding: &Encoding, value: &Value) -> Result<Vec<u8>, Encoding::Error> {
        Ok(value.encode_to_vec())
    }
}

#[cgp_provider(DecoderComponent)]
impl<Encoding, Strategy, Value> Decoder<Encoding, Strategy, Value> for EncodeAsProtobuf
where
    Value: Message + Default,
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanRaiseAsyncError<DecodeError>,
{
    fn decode(_encoding: &Encoding, encoded: &Vec<u8>) -> Result<Value, Encoding::Error> {
        let decoded = Message::decode(encoded.as_ref()).map_err(Encoding::raise_error)?;

        Ok(decoded)
    }
}
