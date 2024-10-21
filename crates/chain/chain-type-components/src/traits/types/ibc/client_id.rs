use core::fmt::Debug;

use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(ClientIdTypeComponent, ProvideClientIdType<Chain>)]
pub trait HasClientIdType<Counterparty>: Async {
    /**
       The client ID of the counterparty chain, that is stored on the local chain.
    */
    type ClientId: Debug + Async;
}

impl<Chain, Counterparty, Provider, ClientId> ProvideClientIdType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, ClientIdTypeComponent, Type = ClientId>,
    ClientId: Debug + Async,
{
    type ClientId = ClientId;
}
