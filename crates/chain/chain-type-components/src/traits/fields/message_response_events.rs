use cgp::prelude::*;

use crate::traits::types::event::HasEventType;
use crate::traits::types::message_response::HasMessageResponseType;

#[cgp_component {
  name: MessageResponseEventsGetterComponent,
  provider: MessageResponseEventsGetter,
  context: Chain,
}]
pub trait HasMessageResponseEvents: HasMessageResponseType + HasEventType {
    fn message_response_events(message_response: &Self::MessageResponse) -> &[Self::Event];
}
