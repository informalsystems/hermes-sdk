use cgp::prelude::*;

#[derive_component(TimeoutUnorderedPacketPayloadTypeComponent, ProvideTimeoutUnorderedPacketPayloadType<Chain>)]
pub trait HasTimeoutUnorderedPacketPayloadType<Counterparty>: Async {
    type TimeoutUnorderedPacketPayload: Async;
}

#[derive_component(PacketReceiptTypeComponent, ProvidePacketReceiptType<Chain>)]
pub trait HasPacketReceiptType<Counterparty>: Async {
    type PacketReceipt: Async;
}
