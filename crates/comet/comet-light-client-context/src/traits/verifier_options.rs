use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use tendermint_light_client_verifier::options::Options;

#[cgp_component {
  name: VerifierOptionsGetterComponent,
  provider: VerifierOptionsGetter,
  context: Client,
}]
pub trait HasVerifierOptions: Async {
    fn verifier_options(&self) -> &Options;
}

impl<Client> VerifierOptionsGetter<Client> for UseContext
where
    Client: Async + HasField<symbol!("verifier_options"), Value = Options>,
{
    fn verifier_options(client: &Client) -> &Options {
        client.get_field(PhantomData)
    }
}
