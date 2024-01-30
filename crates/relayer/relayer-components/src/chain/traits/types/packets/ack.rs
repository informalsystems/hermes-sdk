use cgp_core::prelude::*;

#[derive_component(AckPacketPayloadTypeComponent, ProvideAckPacketPayloadType<Chain>)]
pub trait HasAckPacketPayloadType<Counterparty>: Async {
    type AckPacketPayload: Async;
}
