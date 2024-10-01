use cgp::prelude::*;

#[derive_component(PacketAckTypeComponent, ProvidePacketAckType<Chain>)]
pub trait HasPacketAckType<Counterparty>: Async {
    type PacketAck: Async;
}
