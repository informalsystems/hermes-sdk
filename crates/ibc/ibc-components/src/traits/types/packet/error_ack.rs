use cgp::prelude::*;

#[derive_component(PacketErrorAckTypeComponent, ProvidePacketErrorAckType<Chain>)]
pub trait HasPacketErrorAckType<Counterparty>: Async {
    type PacketErrorAck: Async;
}
