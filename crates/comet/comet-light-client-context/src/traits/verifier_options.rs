use cgp::prelude::*;
use tendermint_light_client_verifier::options::Options;

#[derive_component(VerifierOptionsGetterComponent, VerifierOptionsGetter<Client>)]
pub trait HasVerifierOptions: Async {
    fn verifier_options(&self) -> &Options;
}
