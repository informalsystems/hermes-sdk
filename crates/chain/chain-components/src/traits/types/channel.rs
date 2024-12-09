use cgp::prelude::*;

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

#[cgp_component {
  name: ChannelOpenAckPayloadTypeComponent,
  provider: ProvideChannelOpenAckPayloadType,
  context: Chain,
}]
pub trait HasChannelOpenAckPayloadType<Counterparty> {
    type ChannelOpenAckPayload: Async;
}

#[cgp_component {
  name: ChannelOpenConfirmPayloadTypeComponent,
  provider: ProvideChannelOpenConfirmPayloadType,
  context: Chain,
}]
pub trait HasChannelOpenConfirmPayloadType<Counterparty> {
    type ChannelOpenConfirmPayload: Async;
}

#[cgp_component {
  name: ChannelEndTypeComponent,
  provider: ProvideChannelEndType,
  context: Chain,
}]
pub trait HasChannelEndType<Counterparty>: Async {
    type ChannelEnd: Async;
}
