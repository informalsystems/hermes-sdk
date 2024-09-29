use core::fmt::{Debug, Display};

use cgp::prelude::*;

#[derive_component(ConnectionIdTypeComponent, ProvideConnectionIdType<Chain>)]
pub trait HasConnectionIdType<Counterparty>: Async {
    /**
       The connection ID of the counterparty chain, that is stored on the self
       chain.
    */
    type ConnectionId: Debug + Display + Async;
}
