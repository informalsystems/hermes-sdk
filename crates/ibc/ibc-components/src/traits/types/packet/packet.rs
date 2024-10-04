use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

/// Represents an outgoing packet. Incoming packets will be represented as
/// `Counterparty::Packet`
#[derive_component(PacketTypeComponent, ProvidePacketType<Chain>)]
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
