use hermes_prelude::*;

#[cgp_component {
  name: ReceivePacketPayloadTypeComponent,
  provider: ProvideReceivePacketPayloadType,
  context: Chain,
}]
pub trait HasReceivePacketPayloadType<Counterparty>: Async {
    type ReceivePacketPayload: Async;
}

#[cgp_component {
  name: PacketCommitmentTypeComponent,
  provider: ProvidePacketCommitmentType,
  context: Chain,
}]
pub trait HasPacketCommitmentType<Counterparty>: Async {
    type PacketCommitment: Async;
}
