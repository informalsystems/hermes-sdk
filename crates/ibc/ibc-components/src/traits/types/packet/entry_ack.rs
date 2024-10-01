use cgp::prelude::*;

#[derive_component(PacketEntryAckTypeComponent, ProvidePacketEntryAckType<Chain>)]
pub trait HasPacketEntryAckType<Counterparty, App>: Async {
    type PacketEntryAck: Async;
}
