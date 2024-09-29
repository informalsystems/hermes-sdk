use core::fmt::{Debug, Display};

use cgp::prelude::*;

#[derive_component(ChannelIdTypeComponent, ProvideChannelIdType<Chain>)]
pub trait HasChannelIdType<Counterparty>: Async {
    /**
       The channel ID of the counterparty chain, that is stored on the self
       chain.
    */
    type ChannelId: Debug + Display + Async;
}
