use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use hermes_prelude::*;

#[cgp_component {
  name: PacketNonceTypeComponent,
  provider: ProvidePacketNonceType,
  context: Chain,
}]
pub trait HasPacketNonceType<Counterparty>: Async {
    type PacketNonce: Async;
}

#[cgp_provider(PacketNonceTypeComponent)]
impl<Chain, Counterparty, Provider, PacketNonce> ProvidePacketNonceType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, PacketNonceTypeComponent, Type = PacketNonce>,
    PacketNonce: Async,
{
    type PacketNonce = PacketNonce;
}
