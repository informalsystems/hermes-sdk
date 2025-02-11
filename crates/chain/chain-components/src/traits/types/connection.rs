use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: InitConnectionOptionsTypeComponent,
  provider: ProvideInitConnectionOptionsType,
  context: Chain,
}]
pub trait HasInitConnectionOptionsType<Counterparty>: Async {
    type InitConnectionOptions: Async;
}

pub type InitConnectionOptionsOf<Chain, Counterparty> =
    <Chain as HasInitConnectionOptionsType<Counterparty>>::InitConnectionOptions;

#[cgp_component {
  name: ConnectionOpenInitPayloadTypeComponent,
  provider: ProvideConnectionOpenInitPayloadType,
  context: Chain,
}]
pub trait HasConnectionOpenInitPayloadType<Counterparty>: Async {
    type ConnectionOpenInitPayload: Async;
}

#[cgp_component {
  name: ConnectionOpenTryPayloadTypeComponent,
  provider: ProvideConnectionOpenTryPayloadType,
  context: Chain,
}]
pub trait HasConnectionOpenTryPayloadType<Counterparty>: Async {
    type ConnectionOpenTryPayload: Async;
}

#[cgp_component {
  name: ConnectionOpenAckPayloadTypeComponent,
  provider: ProvideConnectionOpenAckPayloadType,
  context: Chain,
}]
pub trait HasConnectionOpenAckPayloadType<Counterparty>: Async {
    type ConnectionOpenAckPayload: Async;
}

#[cgp_component {
  name: ConnectionOpenConfirmPayloadTypeComponent,
  provider: ProvideConnectionOpenConfirmPayloadType,
  context: Chain,
}]
pub trait HasConnectionOpenConfirmPayloadType<Counterparty>: Async {
    type ConnectionOpenConfirmPayload: Async;
}

#[cgp_component {
  name: ConnectionEndTypeComponent,
  provider: ProvideConnectionEndType,
  context: Chain,
}]
pub trait HasConnectionEndType<Counterparty>: Async {
    type ConnectionEnd: Async;
}

#[cgp_provider(ConnectionEndTypeComponent)]
impl<Chain, Counterparty, Provider, ConnectionEnd> ProvideConnectionEndType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Provider: ProvideType<Chain, ConnectionEndTypeComponent, Type = ConnectionEnd>,
    Chain: Async,
    ConnectionEnd: Async,
{
    type ConnectionEnd = ConnectionEnd;
}
