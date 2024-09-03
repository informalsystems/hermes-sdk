use core::marker::PhantomData;

use cgp::core::Async;

use crate::traits::decode::{CanDecode, Decoder};
use crate::traits::encode::{CanEncode, Encoder};

pub struct EncodeUsingStrategy<Strategy>(pub PhantomData<Strategy>);

impl<Encoding, Strategy, InStrategy, Value> Encoder<Encoding, Strategy, Value>
    for EncodeUsingStrategy<InStrategy>
where
    Encoding: CanEncode<InStrategy, Value>,
    InStrategy: Async,
{
    fn encode(encoding: &Encoding, value: &Value) -> Result<Encoding::Encoded, Encoding::Error> {
        <Encoding as CanEncode<InStrategy, Value>>::encode(encoding, value)
    }
}

impl<Encoding, Strategy, InStrategy, Value> Decoder<Encoding, Strategy, Value>
    for EncodeUsingStrategy<InStrategy>
where
    Encoding: CanDecode<InStrategy, Value>,
    InStrategy: Async,
{
    fn decode(encoding: &Encoding, value: &Encoding::Encoded) -> Result<Value, Encoding::Error> {
        <Encoding as CanDecode<InStrategy, Value>>::decode(encoding, value)
    }
}
