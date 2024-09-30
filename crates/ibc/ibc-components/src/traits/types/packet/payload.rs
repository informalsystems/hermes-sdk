use cgp::prelude::*;

#[derive_component(PacketPayloadHeaderTypeComponent, ProvidePacketPayloadHeaderType<Chain>)]
pub trait HasPacketPayloadHeaderType<Counterparty>: Async {
    type PacketPayloadHeader: Async;
}
