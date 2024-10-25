use cgp::prelude::*;

#[derive_component(MessageResponseTypeComponent, ProvideMessageResponseType<Chain>)]
pub trait HasMessageResponseType: Async {
    type MessageResponse: Async;
}
