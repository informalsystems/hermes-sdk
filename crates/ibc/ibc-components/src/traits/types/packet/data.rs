use cgp::prelude::*;

#[derive_component(PacketDataTypeComponent, ProvidePacketDataType<Chain>)]
pub trait HasPacketDataType<Counterparty, App>: Async {
    type PacketData: Async;
}
