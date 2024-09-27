use core::fmt::{Debug, Display};

use cgp::prelude::*;

#[derive_component(ClientIdTypeComponent, ProvideClientIdType<Chain>)]
pub trait HasClientIdType<Counterparty>: Async {
    /**
       The client ID of the counterparty chain, that is stored on the local chain.
    */
    type ClientId: Debug + Display + Async;
}
