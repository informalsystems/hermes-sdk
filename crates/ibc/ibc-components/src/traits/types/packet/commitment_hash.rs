use cgp::prelude::*;

#[derive_component(PacketPacketCommitmentHashTypeComponent, ProvidePacketCommitmentHashType<Chain>)]
pub trait HasPacketCommitmentHashType<Counterparty>: Async {
    type PacketCommitmentHash: Async;
}
