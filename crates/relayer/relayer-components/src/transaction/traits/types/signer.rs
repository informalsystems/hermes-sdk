use cgp::prelude::*;

#[derive_component(SignerTypeComponent, ProvideSignerType<Chain>)]
pub trait HasSignerType: Async {
    type Signer: Async;
}

pub type SignerOf<Context> = <Context as HasSignerType>::Signer;
