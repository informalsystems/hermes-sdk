use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

/**
   This is an abstract type for an outgoing IBC packet. Incoming IBC packets
   will be represented as `Counterparty::Packet`.

   An IBC packet may contain three fields:
   - Packet nonce, via `HasPacketNonce`
   - Packet headers, via `HasPacketHeader`
   - Packet payloads, via `HasPacketPayloads`
*/
#[cgp_component {
  name: PacketTypeComponent,
  provider: ProvidePacketType,
  context: Chain,
}]
pub trait HasPacketType<Counterparty>: Async {
    type Packet: Async;
}

impl<Chain, Counterparty, Provider, Packet> ProvidePacketType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, PacketTypeComponent, Type = Packet>,
    Packet: Async,
{
    type Packet = Packet;
}
