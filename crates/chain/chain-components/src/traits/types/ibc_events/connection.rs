use cgp::prelude::*;
use hermes_chain_type_components::traits::HasConnectionIdType;

#[cgp_component {
  name: ConnectionOpenTryEventComponent,
  provider: ProvideConnectionOpenTryEvent,
  context: Chain,
}]
pub trait HasConnectionOpenTryEvent<Counterparty>: HasConnectionIdType<Counterparty> {
    type ConnectionOpenTryEvent: Async;

    fn connection_open_try_event_connection_id(
        event: &Self::ConnectionOpenTryEvent,
    ) -> &Self::ConnectionId;
}

#[cgp_component {
  name: ConnectionOpenInitEventComponent,
  provider: ProvideConnectionOpenInitEvent,
  context: Chain,
}]
pub trait HasConnectionOpenInitEvent<Counterparty>: HasConnectionIdType<Counterparty> {
    type ConnectionOpenInitEvent: Async;

    fn connection_open_init_event_connection_id(
        event: &Self::ConnectionOpenInitEvent,
    ) -> &Self::ConnectionId;
}
