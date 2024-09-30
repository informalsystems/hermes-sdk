use cgp::prelude::*;

#[derive_component(PacketAckTypeComponent, ProvidePacketAckType<Chain>)]
pub trait HasPacketAckType<App, Counterparty>: Async {
    type PacketAck: Async;
}
