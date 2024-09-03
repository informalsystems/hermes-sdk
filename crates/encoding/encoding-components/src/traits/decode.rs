use cgp::prelude::*;

use crate::traits::types::encoded::HasEncodedType;

#[derive_component(DecoderComponent, Decoder<Encoding>)]
pub trait CanDecode<Strategy, Value>: HasEncodedType + HasErrorType {
    fn decode(&self, encoded: &Self::Encoded) -> Result<Value, Self::Error>;
}
