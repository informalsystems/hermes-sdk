use cgp::prelude::*;

#[derive_component(PacketNonceTypeComponent, ProvidePacketNonceType<Chain>)]
pub trait HasPacketNonceType<Counterparty>: Async {
    type PacketNonce: Async;
}
