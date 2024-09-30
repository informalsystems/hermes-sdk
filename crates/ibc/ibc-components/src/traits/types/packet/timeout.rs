use cgp::prelude::*;

#[derive_component(PacketTimeoutTypeComponent, ProvidePacketTimeoutType<Chain>)]
pub trait HasPacketTimeoutType<Counterparty>: Async {
    type PacketTimeout: Async;
}
