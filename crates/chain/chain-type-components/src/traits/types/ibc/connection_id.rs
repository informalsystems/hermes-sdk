use core::fmt::{Debug, Display};

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
