use crate::traits::types::event::HasEventType;
use crate::traits::types::message_response::HasMessageResponseType;
use cgp::prelude::*;

#[derive_component(MessageResponseEventsGetterComponent, MessageResponseEventsGetter<Chain>)]
pub trait HasMessageResponseEvents: HasMessageResponseType + HasEventType {
    fn message_response_events(message_response: &Self::MessageResponse) -> &[Self::Event];
}
