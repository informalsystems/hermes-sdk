use alloc::vec::Vec;

use crate::traits::fields::message_response_events::MessageResponseEventsGetter;
use crate::traits::types::event::HasEventType;
use crate::traits::types::message_response::{HasMessageResponseType, ProvideMessageResponseType};

pub struct UseEventsMessageResponse;

impl<Chain> ProvideMessageResponseType<Chain> for UseEventsMessageResponse
where
    Chain: HasEventType,
{
    type MessageResponse = Vec<Chain::Event>;
}

impl<Chain> MessageResponseEventsGetter<Chain> for UseEventsMessageResponse
where
    Chain: HasEventType + HasMessageResponseType<MessageResponse = Vec<Chain::Event>>,
{
    fn message_response_events(message_response: &Vec<Chain::Event>) -> &[Chain::Event] {
        message_response
    }
}
