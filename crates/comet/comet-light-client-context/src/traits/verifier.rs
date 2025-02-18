use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use tendermint_light_client_verifier::Verifier;

#[cgp_component {
  name: VerifierComponent,
  provider: ProvideVerifier,
  context: Client,
}]
pub trait HasVerifier: Async {
    type Verifier: Verifier;

    fn verifier(&self) -> &Self::Verifier;
}

#[cgp_provider(VerifierComponent)]
impl<Client> ProvideVerifier<Client> for UseContext
where
    Client: Async + HasField<symbol!("verifier"), Value: Verifier>,
{
    type Verifier = Client::Value;

    fn verifier(client: &Client) -> &Self::Verifier {
        client.get_field(PhantomData)
    }
}
