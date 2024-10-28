use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::connection_id::HasConnectionIdType;
use hermes_chain_type_components::traits::types::message_response::HasMessageResponseType;

#[derive_component(ConnectionOpenTryEventComponent, ProvideConnectionOpenTryEvent<Chain>)]
pub trait HasConnectionOpenTryEvent<Counterparty>:
    HasMessageResponseType + HasConnectionIdType<Counterparty>
{
    type ConnectionOpenTryEvent: Async;

    fn try_extract_connection_open_try_event(
        response: &Self::MessageResponse,
    ) -> Option<Self::ConnectionOpenTryEvent>;

    fn connection_open_try_event_connection_id(
        event: &Self::ConnectionOpenTryEvent,
    ) -> &Self::ConnectionId;
}

#[derive_component(ConnectionOpenInitEventComponent, ProvideConnectionOpenInitEvent<Chain>)]
pub trait HasConnectionOpenInitEvent<Counterparty>:
    HasMessageResponseType + HasConnectionIdType<Counterparty>
{
    type ConnectionOpenInitEvent: Async;

    fn try_extract_connection_open_init_event(
        response: &Self::MessageResponse,
    ) -> Option<Self::ConnectionOpenInitEvent>;

    fn connection_open_init_event_connection_id(
        event: &Self::ConnectionOpenInitEvent,
    ) -> &Self::ConnectionId;
}
