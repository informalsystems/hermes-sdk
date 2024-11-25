use cgp::prelude::*;
use tendermint_light_client_verifier::Verifier;

#[derive_component(VerifierComponent, ProvideVerifier<Chain>)]
pub trait HasVerifier: Async {
    type Verifier: Verifier;

    fn verifier(&self) -> &Self::Verifier;
}
