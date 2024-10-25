use cgp::prelude::*;

use crate::traits::types::event::HasEventType;
use crate::traits::types::message_response::HasMessageResponseType;

#[derive_component(MessageResponseEventsGetterComponent, MessageResponseEventsGetter<Chain>)]
pub trait HasMessageResponseEvents: HasMessageResponseType + HasEventType {
    fn message_response_events(message_response: &Self::MessageResponse) -> &[Self::Event];
}
