use cgp_core::prelude::*;

use crate::encode::encoded::HasEncodedType;

#[derive_component(DecoderComponent, Decoder<Encoding>)]
pub trait CanDecode<Value>: HasEncodedType + HasErrorType {
    fn decode(&self, encoded: &Self::Encoded) -> Result<Value, Self::Error>;
}
