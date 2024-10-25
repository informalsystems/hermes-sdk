use cgp::prelude::*;

#[derive_component(MessageResponseTypeComponent, ProvideMessageResponseType<Chain>)]
pub trait HasMessageResponseType: Async {
    type MessageResponse: Async;
}

pub type MessageResponseOf<Chain> = <Chain as HasMessageResponseType>::MessageResponse;
