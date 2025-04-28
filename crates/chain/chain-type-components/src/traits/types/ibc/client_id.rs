use core::fmt::{Debug, Display};

use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use hermes_prelude::*;

#[cgp_component {
  name: ClientIdTypeComponent,
  provider: ProvideClientIdType,
  context: Chain,
}]
pub trait HasClientIdType<Counterparty>: Sized + Async {
    /**
       The client ID of the counterparty chain, that is stored on the local chain.
    */
    type ClientId: Debug + Display + Async;
}

#[cgp_provider(ClientIdTypeComponent)]
impl<Chain, Counterparty, Provider, ClientId> ProvideClientIdType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, ClientIdTypeComponent, Type = ClientId>,
    ClientId: Debug + Display + Async,
{
    type ClientId = ClientId;
}
