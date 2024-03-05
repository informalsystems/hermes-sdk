use core::marker::PhantomData;

use cgp_core::Async;

use crate::encode::traits::convert::CanConvert;
use crate::encode::traits::decoder::{CanDecode, Decoder};
use crate::encode::traits::encoder::{CanEncode, Encoder};

pub struct ConvertAndEncode<Raw>(pub PhantomData<Raw>);

impl<Encoding, Value, Raw> Encoder<Encoding, Value> for ConvertAndEncode<Raw>
where
    Encoding: CanEncode<Raw> + CanConvert<Value, Raw>,
    Raw: Async,
{
    fn encode(encoding: &Encoding, value: &Value) -> Result<Encoding::Encoded, Encoding::Error> {
        let raw = encoding.convert(value)?;

        encoding.encode(&raw)
    }
}

impl<Encoding, Value, Raw> Decoder<Encoding, Value> for ConvertAndEncode<Raw>
where
    Encoding: CanDecode<Raw> + CanConvert<Raw, Value>,
    Raw: Async,
{
    fn decode(encoding: &Encoding, encoded: &Encoding::Encoded) -> Result<Value, Encoding::Error> {
        let raw = encoding.decode(encoded)?;

        encoding.convert(&raw)
    }
}
