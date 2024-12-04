use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use tendermint_light_client_verifier::options::Options;

#[derive_component(VerifierOptionsGetterComponent, VerifierOptionsGetter<Client>)]
pub trait HasVerifierOptions: Async {
    fn verifier_options(&self) -> &Options;
}

impl<Client> VerifierOptionsGetter<Client> for UseContext
where
    Client: Async + HasField<symbol!("verifier_options"), Field = Options>,
{
    fn verifier_options(client: &Client) -> &Options {
        client.get_field(PhantomData)
    }
}
