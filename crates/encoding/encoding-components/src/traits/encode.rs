use cgp::prelude::*;

use crate::traits::types::encoded::HasEncodedType;

#[cgp_component {
  name: EncoderComponent,
  provider: Encoder,
  context: Encoding,
}]
pub trait CanEncode<Strategy, Value>: HasEncodedType + HasErrorType {
    fn encode(&self, value: &Value) -> Result<Self::Encoded, Self::Error>;
}
