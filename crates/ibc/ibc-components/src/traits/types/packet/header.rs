use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: PacketHeaderTypeComponent,
  provider: ProvidePacketHeaderType,
  context: Chain,
}]
pub trait HasPacketHeaderType<Counterparty>: Sized + Async {
    type PacketHeader: Async;
}

#[cgp_provider(PacketHeaderTypeComponent)]
impl<Chain, Counterparty, Provider, PacketHeader> ProvidePacketHeaderType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, PacketHeaderTypeComponent, Type = PacketHeader>,
    PacketHeader: Async,
{
    type PacketHeader = PacketHeader;
}
