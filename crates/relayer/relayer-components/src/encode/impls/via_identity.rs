use crate::encode::traits::decoder::{CanDecode, Decoder};
use crate::encode::traits::encoder::{CanEncode, Encoder};
use crate::encode::types::via::Via;

pub struct Identity;

pub struct EncodeViaIdentity;

impl<Encoding, Value> Encoder<Encoding, Via<Identity, Value>> for EncodeViaIdentity
where
    Encoding: CanEncode<Value>,
{
    fn encode(
        encoding: &Encoding,
        value: &Via<Identity, Value>,
    ) -> Result<Encoding::Encoded, Encoding::Error> {
        encoding.encode(&value.value)
    }
}

impl<Encoding, Value> Decoder<Encoding, Via<Identity, Value>> for EncodeViaIdentity
where
    Encoding: CanDecode<Value>,
{
    fn decode(
        encoding: &Encoding,
        encoded: &Encoding::Encoded,
    ) -> Result<Via<Identity, Value>, Encoding::Error> {
        let value: Value = encoding.decode(encoded)?;
        Ok(value.into())
    }
}
