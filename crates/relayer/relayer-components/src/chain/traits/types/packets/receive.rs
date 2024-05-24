use cgp_core::prelude::*;

#[derive_component(ReceivePacketPayloadTypeComponent, ProvideReceivePacketPayloadType<Chain>)]
pub trait HasReceivePacketPayloadType<Counterparty>: Async {
    type ReceivePacketPayload: Async;
}

#[derive_component(PacketCommitmentTypeComponent, ProvidePacketCommitmentType<Chain>)]
pub trait HasPacketCommitmentType<Counterparty>: Async {
    type PacketCommitment: Async;
}
