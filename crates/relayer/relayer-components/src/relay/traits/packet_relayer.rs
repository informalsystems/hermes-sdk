use cgp::prelude::*;

use crate::relay::traits::HasRelayPacketType;

#[cgp_component {
  provider: PacketRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanRelayPacket: HasRelayPacketType {
    async fn relay_packet(&self, packet: &Self::Packet) -> Result<(), Self::Error>;
}
