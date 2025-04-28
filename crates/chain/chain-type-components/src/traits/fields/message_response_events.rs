use hermes_prelude::*;

use crate::traits::{HasEventType, HasMessageResponseType};

#[cgp_component {
  provider: MessageResponseEventsGetter,
  context: Chain,
}]
pub trait HasMessageResponseEvents: HasMessageResponseType + HasEventType {
    fn message_response_events(message_response: &Self::MessageResponse) -> &[Self::Event];
}
