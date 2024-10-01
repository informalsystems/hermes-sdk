use cgp::prelude::*;

#[derive_component(PacketAckTypeComponent, ProvidePacketAckType<Chain>)]
pub trait HasPacketAckType<Counterparty, App>: Async {
    type PacketAck: Async;
}
