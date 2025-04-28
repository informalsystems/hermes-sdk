use hermes_prelude::*;

#[cgp_component {
  name: MessageResponseTypeComponent,
  provider: ProvideMessageResponseType,
  context: Chain,
}]
pub trait HasMessageResponseType: Async {
    type MessageResponse: Async;
}

pub type MessageResponseOf<Chain> = <Chain as HasMessageResponseType>::MessageResponse;
