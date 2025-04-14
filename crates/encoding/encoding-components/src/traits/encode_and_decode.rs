use crate::traits::{CanDecode, CanEncode};

pub trait CanEncodeAndDecode<Strategy, Value>:
    CanEncode<Strategy, Value> + CanDecode<Strategy, Value>
{
}

impl<Encoding, Strategy, Value> CanEncodeAndDecode<Strategy, Value> for Encoding where
    Encoding: CanEncode<Strategy, Value> + CanDecode<Strategy, Value>
{
}
