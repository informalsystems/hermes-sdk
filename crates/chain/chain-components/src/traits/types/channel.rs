use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use hermes_prelude::*;

#[cgp_component {
  name: InitChannelOptionsTypeComponent,
  provider: ProvideInitChannelOptionsType,
  context: Chain,
}]
pub trait HasInitChannelOptionsType<Counterparty> {
    type InitChannelOptions: Async;
}

pub type InitChannelOptions<Chain, Counterparty> =
    <Chain as HasInitChannelOptionsType<Counterparty>>::InitChannelOptions;

/**
    Payload that contains necessary counterparty information such as proofs and parameters
    in order for a self chain to build a channel handshake message.
*/
#[cgp_component {
  name: ChannelOpenTryPayloadTypeComponent,
  provider: ProvideChannelOpenTryPayloadType,
  context: Chain,
}]
pub trait HasChannelOpenTryPayloadType<Counterparty> {
    type ChannelOpenTryPayload: Async;
}

pub type ChannelOpenTryPayloadOf<Chain, Counterparty> =
    <Chain as HasChannelOpenTryPayloadType<Counterparty>>::ChannelOpenTryPayload;

#[cgp_component {
  name: ChannelOpenAckPayloadTypeComponent,
  provider: ProvideChannelOpenAckPayloadType,
  context: Chain,
}]
pub trait HasChannelOpenAckPayloadType<Counterparty> {
    type ChannelOpenAckPayload: Async;
}

pub type ChannelOpenAckPayloadOf<Chain, Counterparty> =
    <Chain as HasChannelOpenAckPayloadType<Counterparty>>::ChannelOpenAckPayload;

#[cgp_component {
  name: ChannelOpenConfirmPayloadTypeComponent,
  provider: ProvideChannelOpenConfirmPayloadType,
  context: Chain,
}]
pub trait HasChannelOpenConfirmPayloadType<Counterparty> {
    type ChannelOpenConfirmPayload: Async;
}

pub type ChannelOpenConfirmPayloadOf<Chain, Counterparty> =
    <Chain as HasChannelOpenConfirmPayloadType<Counterparty>>::ChannelOpenConfirmPayload;

#[cgp_component {
  name: ChannelEndTypeComponent,
  provider: ProvideChannelEndType,
  context: Chain,
}]
pub trait HasChannelEndType<Counterparty>: Async {
    type ChannelEnd: Async;
}

#[cgp_provider(ChannelEndTypeComponent)]
impl<Chain, Counterparty, Provider, ChannelEnd> ProvideChannelEndType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Provider: ProvideType<Chain, ChannelEndTypeComponent, Type = ChannelEnd>,
    Chain: Async,
    ChannelEnd: Async,
{
    type ChannelEnd = ChannelEnd;
}
