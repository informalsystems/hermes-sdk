use alloc::vec::Vec;

use hermes_prelude::*;

use crate::relay::traits::HasRelayPacketType;

#[cgp_component {
  provider: PacketRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanRelayPacket: HasRelayPacketType {
    async fn relay_packet(&self, packet: &Self::Packet) -> Result<(), Self::Error>;
}

#[cgp_component {
  provider: BatchPacketsRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanRelayBatchPackets: HasRelayPacketType {
    async fn relay_packets(&self, packet: Vec<&Self::Packet>) -> Result<(), Self::Error>;
}
