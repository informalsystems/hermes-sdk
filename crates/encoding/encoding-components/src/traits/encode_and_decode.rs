use crate::traits::decoder::CanDecode;
use crate::traits::encoder::CanEncode;

pub trait CanEncodeAndDecode<Strategy, Value>:
    CanEncode<Strategy, Value> + CanDecode<Strategy, Value>
{
}

impl<Encoding, Strategy, Value> CanEncodeAndDecode<Strategy, Value> for Encoding where
    Encoding: CanEncode<Strategy, Value> + CanDecode<Strategy, Value>
{
}
