use cgp_core::prelude::*;

#[derive_component(NonceTypeComponent, ProvideNonceType<Chain>)]
pub trait HasNonceType: Async {
    type Nonce: Async;
}
