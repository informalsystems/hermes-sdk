use core::marker::PhantomData;

use cgp::core::component::UseContext;
use hermes_prelude::*;
use tendermint_light_client_verifier::options::Options;

#[cgp_component {
  provider: VerifierOptionsGetter,
  context: Client,
}]
pub trait HasVerifierOptions: Async {
    fn verifier_options(&self) -> &Options;
}

#[cgp_provider(VerifierOptionsGetterComponent)]
impl<Client> VerifierOptionsGetter<Client> for UseFields
where
    Client: Async + HasField<symbol!("verifier_options"), Value = Options>,
{
    fn verifier_options(client: &Client) -> &Options {
        client.get_field(PhantomData)
    }
}
