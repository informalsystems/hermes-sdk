use cgp_core::prelude::*;

use crate::chain::traits::types::ibc::HasIbcChainTypes;

pub trait HasConnectionOpenTryEvent<Counterparty>: HasIbcChainTypes<Counterparty> {
    type ConnectionOpenTryEvent: Async;

    fn try_extract_connection_open_try_event(
        event: Self::Event,
    ) -> Option<Self::ConnectionOpenTryEvent>;

    fn connection_open_try_event_connection_id(
        event: &Self::ConnectionOpenTryEvent,
    ) -> &Self::ConnectionId;
}

#[derive_component(ConnectionOpenInitEventComponent, ProvideConnectionOpenInitEvent<Chain>)]
pub trait HasConnectionOpenInitEvent<Counterparty>: HasIbcChainTypes<Counterparty> {
    type ConnectionOpenInitEvent: Async;

    fn try_extract_connection_open_init_event(
        event: Self::Event,
    ) -> Option<Self::ConnectionOpenInitEvent>;

    fn connection_open_init_event_connection_id(
        event: &Self::ConnectionOpenInitEvent,
    ) -> &Self::ConnectionId;
}
