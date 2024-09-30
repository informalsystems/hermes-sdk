use cgp::prelude::*;

#[derive_component(PacketDataTypeComponent, ProvidePacketDataType<Chain>)]
pub trait HasPacketDataType<App, Counterparty>: Async {
    type PacketData: Async;
}
