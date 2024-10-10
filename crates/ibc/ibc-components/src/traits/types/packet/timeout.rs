use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(PacketTimeoutTypeComponent, ProvidePacketTimeoutType<Chain>)]
pub trait HasPacketTimeoutType<Counterparty>: Async {
    type PacketTimeout: Async;
}

impl<Chain, Counterparty, Provider, PacketTimeout> ProvidePacketTimeoutType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, PacketTimeoutTypeComponent, Type = PacketTimeout>,
    PacketTimeout: Async,
{
    type PacketTimeout = PacketTimeout;
}
