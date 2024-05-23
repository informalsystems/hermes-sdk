use cgp_core::prelude::*;

use crate::chain::traits::types::ibc::HasIbcChainTypes;

#[derive_component(ChannelOpenInitEventComponent, ProvideChannelOpenInitEvent<Chain>)]
pub trait HasChannelOpenInitEvent<Counterparty>: HasIbcChainTypes<Counterparty> {
    type ChannelOpenInitEvent: Async;

    fn try_extract_channel_open_init_event(
        event: Self::Event,
    ) -> Option<Self::ChannelOpenInitEvent>;

    fn channel_open_init_event_channel_id(event: &Self::ChannelOpenInitEvent) -> &Self::ChannelId;
}

#[derive_component(ChannelOpenTryEventComponent, ProvideChannelOpenTryEvent<Chain>)]
pub trait HasChannelOpenTryEvent<Counterparty>: HasIbcChainTypes<Counterparty> {
    type ChannelOpenTryEvent: Async;

    fn try_extract_channel_open_try_event(event: Self::Event) -> Option<Self::ChannelOpenTryEvent>;

    fn channel_open_try_event_channel_id(event: &Self::ChannelOpenTryEvent) -> &Self::ChannelId;
}
