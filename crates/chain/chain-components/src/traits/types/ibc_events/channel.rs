use cgp::prelude::*;
use hermes_chain_type_components::traits::HasChannelIdType;

#[cgp_component {
  name: ChannelOpenInitEventComponent,
  provider: ProvideChannelOpenInitEvent,
  context: Chain,
}]
pub trait HasChannelOpenInitEvent<Counterparty>: HasChannelIdType<Counterparty> {
    type ChannelOpenInitEvent: Async;

    fn channel_open_init_event_channel_id(event: &Self::ChannelOpenInitEvent) -> &Self::ChannelId;
}

#[cgp_component {
  name: ChannelOpenTryEventComponent,
  provider: ProvideChannelOpenTryEvent,
  context: Chain,
}]
pub trait HasChannelOpenTryEvent<Counterparty>: HasChannelIdType<Counterparty> {
    type ChannelOpenTryEvent: Async;

    fn channel_open_try_event_channel_id(event: &Self::ChannelOpenTryEvent) -> &Self::ChannelId;
}
