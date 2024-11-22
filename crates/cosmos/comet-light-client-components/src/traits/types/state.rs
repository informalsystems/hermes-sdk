use cgp::prelude::*;

#[derive_component(VerifierStateTypeComponent, ProvideVerifierStateType<Chain>)]
pub trait HasVerifierStateType: Async {
    type VerifierState: Async;
}
