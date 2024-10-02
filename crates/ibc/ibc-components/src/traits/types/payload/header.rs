use cgp::prelude::*;

#[derive_component(PayloadHeaderTypeComponent, ProvidePayloadHeaderType<Chain>)]
pub trait HasPayloadHeaderType<Counterparty>: Async {
    type PayloadHeader: Async;
}
