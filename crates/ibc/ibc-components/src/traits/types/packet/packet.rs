use cgp::prelude::*;

/// Represents an outgoing packet. Incoming packets will be represented as
/// `Counterparty::Packet`
#[derive_component(PacketTypeComponent, ProvideHeaderType<Chain>)]
pub trait HasPacketType<Counterparty>: Async {
    type Packet: Async;
}