use cgp::prelude::*;

#[derive_component(PayloadAckTypeComponent, ProvidePayloadAckType<Chain>)]
pub trait HasPayloadAckType<Counterparty, App>: Async {
    type PayloadAck: Async;
}
