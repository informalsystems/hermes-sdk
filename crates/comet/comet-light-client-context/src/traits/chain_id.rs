use core::marker::PhantomData;

use cgp::core::component::UseContext;
use hermes_prelude::*;

#[cgp_component {
  provider: ChainIdGetter,
  context: Client,
}]
pub trait HasChainId: Async {
    fn chain_id(&self) -> &String;
}

#[cgp_provider(ChainIdGetterComponent)]
impl<Client> ChainIdGetter<Client> for UseFields
where
    Client: Async + HasField<symbol!("chain_id"), Value = String>,
{
    fn chain_id(client: &Client) -> &String {
        client.get_field(PhantomData::<symbol!("chain_id")>)
    }
}
