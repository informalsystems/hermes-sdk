use cgp_core::HasErrorType;

use crate::traits::decoder::Decoder;
use crate::traits::encoded::HasEncodedType;
use crate::traits::encoder::Encoder;

pub struct ReturnEncoded;

impl<Encoding, Strategy, Value> Encoder<Encoding, Strategy, Value> for ReturnEncoded
where
    Encoding: HasEncodedType<Encoded = Value> + HasErrorType,
    Value: Clone,
{
    fn encode(_encoding: &Encoding, value: &Value) -> Result<Value, Encoding::Error> {
        Ok(value.clone())
    }
}

impl<Encoding, Strategy, Value> Decoder<Encoding, Strategy, Value> for ReturnEncoded
where
    Encoding: HasEncodedType<Encoded = Value> + HasErrorType,
    Value: Clone,
{
    fn decode(_encoding: &Encoding, value: &Value) -> Result<Value, Encoding::Error> {
        Ok(value.clone())
    }
}
