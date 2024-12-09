use cgp::prelude::*;

#[cgp_component {
  name: TimeoutUnorderedPacketPayloadTypeComponent,
  provider: ProvideTimeoutUnorderedPacketPayloadType,
  context: Chain,
}]
pub trait HasTimeoutUnorderedPacketPayloadType<Counterparty>: Async {
    type TimeoutUnorderedPacketPayload: Async;
}

#[cgp_component {
  name: PacketReceiptTypeComponent,
  provider: ProvidePacketReceiptType,
  context: Chain,
}]
pub trait HasPacketReceiptType<Counterparty>: Async {
    type PacketReceipt: Async;
}
