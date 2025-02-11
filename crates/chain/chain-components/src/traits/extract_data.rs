use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;
use hermes_chain_type_components::traits::fields::message_response_events::HasMessageResponseEvents;
use hermes_chain_type_components::traits::types::event::HasEventType;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;

#[cgp_component {
    provider: MessageResponseExtractor,
    context: Chain,
}]
pub trait CanExtractFromMessageResponse<Data>: HasMessageResponseType {
    fn try_extract_from_message_response(
        &self,
        _tag: PhantomData<Data>,
        message_response: &Self::MessageResponse,
    ) -> Option<Data>;
}

#[cgp_component {
    provider: EventExtractor,
    context: Chain,
}]
pub trait CanExtractFromEvent<Data>: HasEventType {
    fn try_extract_from_event(&self, _tag: PhantomData<Data>, event: &Self::Event) -> Option<Data>;
}

pub struct ExtractFromMessageResponseViaEvents;

#[cgp_provider(MessageResponseExtractorComponent)]
impl<Chain, Data> MessageResponseExtractor<Chain, Data> for ExtractFromMessageResponseViaEvents
where
    Chain: HasMessageResponseEvents + CanExtractFromEvent<Data>,
{
    fn try_extract_from_message_response(
        chain: &Chain,
        tag: PhantomData<Data>,
        message_response: &Chain::MessageResponse,
    ) -> Option<Data> {
        Chain::message_response_events(message_response)
            .iter()
            .find_map(|event| chain.try_extract_from_event(tag, event))
    }
}

#[cgp_provider(MessageResponseExtractorComponent)]
impl<Chain, Data, Components> MessageResponseExtractor<Chain, Data> for UseDelegate<Components>
where
    Chain: HasMessageResponseType,
    Components: DelegateComponent<Data>,
    Components::Delegate: MessageResponseExtractor<Chain, Data>,
{
    fn try_extract_from_message_response(
        chain: &Chain,
        tag: PhantomData<Data>,
        message_response: &Chain::MessageResponse,
    ) -> Option<Data> {
        Components::Delegate::try_extract_from_message_response(chain, tag, message_response)
    }
}

#[cgp_provider(EventExtractorComponent)]
impl<Chain, Data, Components> EventExtractor<Chain, Data> for UseDelegate<Components>
where
    Chain: HasEventType,
    Components: DelegateComponent<Data>,
    Components::Delegate: EventExtractor<Chain, Data>,
{
    fn try_extract_from_event(
        chain: &Chain,
        tag: PhantomData<Data>,
        event: &Chain::Event,
    ) -> Option<Data> {
        Components::Delegate::try_extract_from_event(chain, tag, event)
    }
}
