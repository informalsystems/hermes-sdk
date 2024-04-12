use crate::traits::decoder::CanDecode;
use crate::traits::encoder::CanEncode;

pub trait CanEncodeAndDecode<Value>: CanEncode<Value> + CanDecode<Value> {}

impl<Encoding, Value> CanEncodeAndDecode<Value> for Encoding where
    Encoding: CanEncode<Value> + CanDecode<Value>
{
}
