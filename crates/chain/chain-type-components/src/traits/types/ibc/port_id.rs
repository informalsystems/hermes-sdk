use core::fmt::{Debug, Display};

use hermes_prelude::*;

#[cgp_component {
  name: PortIdTypeComponent,
  provider: ProvidePortIdType,
  context: Chain,
}]
pub trait HasPortIdType<Counterparty>: Async {
    /**
       The port ID of the counterparty chain, that is stored on the self
       chain.
    */
    type PortId: Debug + Display + Async;
}
