use cgp::prelude::*;

#[derive_component(AckPacketPayloadTypeComponent, ProvideAckPacketPayloadType<Chain>)]
pub trait HasAckPacketPayloadType<Counterparty>: Async {
    type AckPacketPayload: Async;
}

#[derive_component(AcknowledgementTypeComponent, ProvideAcknowledgementType<Chain>)]
pub trait HasAcknowledgementType<Counterparty>: Async {
    type Acknowledgement: Async;
}

pub type AcknowledgementOf<Chain, Counterparty> =
    <Chain as HasAcknowledgementType<Counterparty>>::Acknowledgement;
