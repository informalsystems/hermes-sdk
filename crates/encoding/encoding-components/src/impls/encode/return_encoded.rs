use cgp::core::error::HasAsyncErrorType;

use crate::traits::decode::Decoder;
use crate::traits::encode::Encoder;
use crate::traits::types::encoded::HasEncodedType;

pub struct ReturnEncoded;

impl<Encoding, Strategy, Value> Encoder<Encoding, Strategy, Value> for ReturnEncoded
where
    Encoding: HasEncodedType<Encoded = Value> + HasAsyncErrorType,
    Value: Clone,
{
    fn encode(_encoding: &Encoding, value: &Value) -> Result<Value, Encoding::Error> {
        Ok(value.clone())
    }
}

impl<Encoding, Strategy, Value> Decoder<Encoding, Strategy, Value> for ReturnEncoded
where
    Encoding: HasEncodedType<Encoded = Value> + HasAsyncErrorType,
    Value: Clone,
{
    fn decode(_encoding: &Encoding, value: &Value) -> Result<Value, Encoding::Error> {
        Ok(value.clone())
    }
}
