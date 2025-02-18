use cgp::prelude::*;

use crate::traits::decode::{Decoder, DecoderComponent};
use crate::traits::encode::{Encoder, EncoderComponent};
use crate::traits::types::encoded::HasEncodedType;

pub struct ReturnEncoded;

#[cgp_provider(EncoderComponent)]
impl<Encoding, Strategy, Value> Encoder<Encoding, Strategy, Value> for ReturnEncoded
where
    Encoding: HasEncodedType<Encoded = Value> + HasAsyncErrorType,
    Value: Clone,
{
    fn encode(_encoding: &Encoding, value: &Value) -> Result<Value, Encoding::Error> {
        Ok(value.clone())
    }
}

#[cgp_provider(DecoderComponent)]
impl<Encoding, Strategy, Value> Decoder<Encoding, Strategy, Value> for ReturnEncoded
where
    Encoding: HasEncodedType<Encoded = Value> + HasAsyncErrorType,
    Value: Clone,
{
    fn decode(_encoding: &Encoding, value: &Value) -> Result<Value, Encoding::Error> {
        Ok(value.clone())
    }
}
