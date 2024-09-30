use cgp::prelude::*;

#[derive_component(PacketRawDataTypeComponent, ProvidePacketRawDataType<Chain>)]
pub trait HasPacketRawDataType<Counterparty>: Async {
    type PacketRawData: Async;
}
