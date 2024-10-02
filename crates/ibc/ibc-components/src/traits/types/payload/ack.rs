use cgp::prelude::*;

#[derive_component(PacketEntryAckTypeComponent, ProvidePacketEntryAckType<Chain>)]
pub trait HasPayloadAckType<Counterparty, App>: Async {
    type PayloadAck: Async;
}
