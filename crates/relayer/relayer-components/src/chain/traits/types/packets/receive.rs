use cgp_core::prelude::*;

#[derive_component(ReceivePacketPayloadTypeComponent, ProvideReceivePacketPayloadType<Chain>)]
pub trait HasReceivePacketPayloadType<Counterparty>: Async {
    type ReceivePacketPayload: Async;
}
