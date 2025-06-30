use core::fmt::Debug;

use hermes_prelude::*;

#[cgp_component {
  name: MessageResponseTypeComponent,
  provider: ProvideMessageResponseType,
  context: Chain,
}]
pub trait HasMessageResponseType: Async {
    type MessageResponse: Async + Debug;
}

pub type MessageResponseOf<Chain> = <Chain as HasMessageResponseType>::MessageResponse;
