use cgp::prelude::*;

#[derive_component(PacketHeaderTypeComponent, ProvidePacketHeaderType<Chain>)]
pub trait HasPacketHeaderType<Counterparty>: Async {
    type PacketHeader: Async;
}
