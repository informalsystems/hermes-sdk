use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;

#[derive_component(ChannelOpenInitEventComponent, ProvideChannelOpenInitEvent<Chain>)]
pub trait HasChannelOpenInitEvent<Counterparty>:
    HasMessageResponseType + HasChannelIdType<Counterparty>
{
    type ChannelOpenInitEvent: Async;

    fn try_extract_channel_open_init_event(
        event: &Self::MessageResponse,
    ) -> Option<Self::ChannelOpenInitEvent>;

    fn channel_open_init_event_channel_id(event: &Self::ChannelOpenInitEvent) -> &Self::ChannelId;
}

#[derive_component(ChannelOpenTryEventComponent, ProvideChannelOpenTryEvent<Chain>)]
pub trait HasChannelOpenTryEvent<Counterparty>:
    HasMessageResponseType + HasChannelIdType<Counterparty>
{
    type ChannelOpenTryEvent: Async;

    fn try_extract_channel_open_try_event(
        event: &Self::MessageResponse,
    ) -> Option<Self::ChannelOpenTryEvent>;

    fn channel_open_try_event_channel_id(event: &Self::ChannelOpenTryEvent) -> &Self::ChannelId;
}
