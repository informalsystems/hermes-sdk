use cgp::prelude::*;

#[cgp_component {
  name: AckPacketPayloadTypeComponent,
  provider: ProvideAckPacketPayloadType,
  context: Chain,
}]
pub trait HasAckPacketPayloadType<Counterparty>: Async {
    type AckPacketPayload: Async;
}

#[cgp_component {
  name: AcknowledgementTypeComponent,
  provider: ProvideAcknowledgementType,
  context: Chain,
}]
pub trait HasAcknowledgementType<Counterparty>: Async {
    type Acknowledgement: Async;
}

pub type AcknowledgementOf<Chain, Counterparty> =
    <Chain as HasAcknowledgementType<Counterparty>>::Acknowledgement;
