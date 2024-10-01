use cgp::prelude::*;

#[derive_component(PacketAckTypeComponent, ProvidePacketAckType<Chain>)]
pub trait HasPacketEntryAckType<Counterparty, App>: Async {
    type PacketEntryAck: Async;
}
