use cgp::prelude::*;

use crate::traits::types::encoded::HasEncodedType;

#[cgp_component {
  provider: Decoder,
  context: Encoding,
}]
pub trait CanDecode<Strategy, Value>: HasEncodedType + HasAsyncErrorType {
    fn decode(&self, encoded: &Self::Encoded) -> Result<Value, Self::Error>;
}
