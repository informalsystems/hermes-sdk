use hermes_prelude::*;

use crate::traits::HasEncodeBufferType;

#[cgp_component {
  provider: MutEncoder,
  context: Encoding,
}]
pub trait CanEncodeMut<Strategy, Value>: HasEncodeBufferType + HasAsyncErrorType {
    fn encode_mut(&self, value: &Value, buffer: &mut Self::EncodeBuffer)
        -> Result<(), Self::Error>;
}
