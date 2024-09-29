use core::fmt::{Debug, Display};

use cgp::prelude::*;

#[derive_component(PortIdTypeComponent, ProvidePortIdType<Chain>)]
pub trait HasPortIdType<Counterparty>: Async {
    /**
       The port ID of the counterparty chain, that is stored on the self
       chain.
    */
    type PortId: Debug + Display + Async;
}
