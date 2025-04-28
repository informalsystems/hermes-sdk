use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use hermes_prelude::*;

#[cgp_component {
  name: PacketTimeoutTypeComponent,
  provider: ProvidePacketTimeoutType,
  context: Chain,
}]
pub trait HasPacketTimeoutType<Counterparty>: Async {
    type PacketTimeout: Async;
}

#[cgp_provider(PacketTimeoutTypeComponent)]
impl<Chain, Counterparty, Provider, PacketTimeout> ProvidePacketTimeoutType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, PacketTimeoutTypeComponent, Type = PacketTimeout>,
    PacketTimeout: Async,
{
    type PacketTimeout = PacketTimeout;
}
