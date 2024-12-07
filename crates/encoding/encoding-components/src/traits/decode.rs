use cgp::prelude::*;

use crate::traits::types::encoded::HasEncodedType;

#[cgp_component {
  name: DecoderComponent,
  provider: Decoder,
  context: Encoding,
}]
pub trait CanDecode<Strategy, Value>: HasEncodedType + HasErrorType {
    fn decode(&self, encoded: &Self::Encoded) -> Result<Value, Self::Error>;
}
