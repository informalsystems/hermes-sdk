use cgp_core::prelude::*;

use crate::traits::types::encoded::HasEncodedType;

#[derive_component(EncoderComponent, Encoder<Encoding>)]
pub trait CanEncode<Strategy, Value>: HasEncodedType + HasErrorType {
    fn encode(&self, value: &Value) -> Result<Self::Encoded, Self::Error>;
}
