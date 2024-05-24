use cgp_core::prelude::*;

#[derive_component(AcknowledgementTypeComponent, ProvideAcknowledgementType<Chain>)]
pub trait HasAcknowledgementType<Counterparty>: Async {
    type Acknowledgement: Async;
}

pub type AcknowledgementOf<Chain, Counterparty> =
    <Chain as HasAcknowledgementType<Counterparty>>::Acknowledgement;
