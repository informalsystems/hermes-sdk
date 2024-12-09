use cgp::prelude::*;

#[cgp_component {
  name: NonceTypeComponent,
  provider: ProvideNonceType,
  context: Chain,
}]
pub trait HasNonceType: Async {
    type Nonce: Async;
}
