use cgp_core::Async;

use crate::chain::traits::types::ibc::HasIbcChainTypes;

pub trait HasChannelOpenInitEvent<Counterparty>: HasIbcChainTypes<Counterparty> {
    type ChannelOpenInitEvent: Async;

    fn try_extract_channel_open_init_event(
        event: Self::Event,
    ) -> Option<Self::ChannelOpenInitEvent>;

    fn channel_open_init_event_channel_id(event: &Self::ChannelOpenInitEvent) -> &Self::ChannelId;
}

pub trait HasChannelOpenTryEvent<Counterparty>: HasIbcChainTypes<Counterparty> {
    type ChannelOpenTryEvent: Async;

    fn try_extract_channel_open_try_event(event: Self::Event) -> Option<Self::ChannelOpenTryEvent>;

    fn channel_open_try_event_channel_id(event: &Self::ChannelOpenTryEvent) -> &Self::ChannelId;
}
