use cgp::prelude::*;

#[cgp_component {
  name: SignerTypeComponent,
  provider: ProvideSignerType,
  context: Chain,
}]
pub trait HasSignerType: Async {
    type Signer: Async;
}

pub type SignerOf<Context> = <Context as HasSignerType>::Signer;
