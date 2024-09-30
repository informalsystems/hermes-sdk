use cgp::prelude::*;

#[derive_component(PacketEntryHeaderTypeComponent, ProvideEntryPayloadHeaderType<Chain>)]
pub trait HasPacketEntryHeaderType<Counterparty>: Async {
    type PacketEntryHeader: Async;
}
