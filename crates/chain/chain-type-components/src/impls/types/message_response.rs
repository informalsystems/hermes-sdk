use alloc::vec::Vec;

use cgp::prelude::*;

use crate::traits::fields::message_response_events::{
    MessageResponseEventsGetter, MessageResponseEventsGetterComponent,
};
use crate::traits::types::event::HasEventType;
use crate::traits::types::message_response::{
    HasMessageResponseType, MessageResponseTypeComponent, ProvideMessageResponseType,
};

pub struct UseEventsMessageResponse;

#[cgp_provider(MessageResponseTypeComponent)]
impl<Chain> ProvideMessageResponseType<Chain> for UseEventsMessageResponse
where
    Chain: HasEventType,
{
    type MessageResponse = Vec<Chain::Event>;
}

#[cgp_provider(MessageResponseEventsGetterComponent)]
impl<Chain> MessageResponseEventsGetter<Chain> for UseEventsMessageResponse
where
    Chain: HasEventType + HasMessageResponseType<MessageResponse = Vec<Chain::Event>>,
{
    fn message_response_events(message_response: &Vec<Chain::Event>) -> &[Chain::Event] {
        message_response
    }
}
