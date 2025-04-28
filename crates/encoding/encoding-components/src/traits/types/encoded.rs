use hermes_prelude::*;

#[cgp_component {
  name: EncodedTypeComponent,
  provider: ProvideEncodedType,
  context: Encoding,
}]
pub trait HasEncodedType: Async {
    type Encoded: Async;
}
