use cgp::prelude::*;

#[derive_component(PacketRawAckTypeComponent, ProvidePacketRawAckType<Chain>)]
pub trait HasPacketRawAckType<Counterparty>: Async {
    type PacketRawAck: Async;
}
