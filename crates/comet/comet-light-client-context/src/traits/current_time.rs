use core::marker::PhantomData;

use cgp::core::component::UseContext;
use hermes_prelude::*;
use tendermint_light_client_verifier::types::Time;

#[cgp_component {
  provider: CurrentTimeGetter,
  context: Client,
}]
pub trait HasCurrentTime: Async {
    fn current_time(&self) -> Time;
}

#[cgp_provider(CurrentTimeGetterComponent)]
impl<Client: Async> CurrentTimeGetter<Client> for UseContext
where
    Client: HasField<symbol!("current_time"), Value = Time>,
{
    fn current_time(client: &Client) -> Time {
        *client.get_field(PhantomData)
    }
}
