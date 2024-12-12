use core::fmt::{Debug, Display};

use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: ConnectionIdTypeComponent,
  provider: ProvideConnectionIdType,
  context: Chain,
}]
pub trait HasConnectionIdType<Counterparty>: Async {
    /**
       The connection ID of the counterparty chain, that is stored on the self
       chain.
    */
    type ConnectionId: Debug + Display + Async;
}

impl<Chain, Counterparty, Provider, ConnectionId> ProvideConnectionIdType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, ConnectionIdTypeComponent, Type = ConnectionId>,
    ConnectionId: Debug + Display + Async,
{
    type ConnectionId = ConnectionId;
}
