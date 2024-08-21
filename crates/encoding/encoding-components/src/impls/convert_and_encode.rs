use core::marker::PhantomData;

use cgp_core::Async;

use crate::traits::convert::CanConvert;
use crate::traits::decode::{CanDecode, Decoder};
use crate::traits::encode::{CanEncode, Encoder};

pub struct ConvertAndEncode<Raw>(pub PhantomData<Raw>);

impl<Encoding, Strategy, Value, Raw> Encoder<Encoding, Strategy, Value> for ConvertAndEncode<Raw>
where
    Encoding: CanEncode<Strategy, Raw> + CanConvert<Value, Raw>,
    Raw: Async,
{
    fn encode(encoding: &Encoding, value: &Value) -> Result<Encoding::Encoded, Encoding::Error> {
        let raw = encoding.convert(value)?;

        encoding.encode(&raw)
    }
}

impl<Encoding, Strategy, Value, Raw> Decoder<Encoding, Strategy, Value> for ConvertAndEncode<Raw>
where
    Encoding: CanDecode<Strategy, Raw> + CanConvert<Raw, Value>,
    Raw: Async,
{
    fn decode(encoding: &Encoding, encoded: &Encoding::Encoded) -> Result<Value, Encoding::Error> {
        let raw = encoding.decode(encoded)?;

        encoding.convert(&raw)
    }
}
