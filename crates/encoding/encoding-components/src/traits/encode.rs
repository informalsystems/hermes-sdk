use hermes_prelude::*;

use crate::traits::HasEncodedType;

#[cgp_component {
  provider: Encoder,
  context: Encoding,
}]
pub trait CanEncode<Strategy, Value>: HasEncodedType + HasAsyncErrorType {
    fn encode(&self, value: &Value) -> Result<Self::Encoded, Self::Error>;
}
