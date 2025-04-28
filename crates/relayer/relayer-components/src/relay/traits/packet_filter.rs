use hermes_prelude::*;

use crate::relay::traits::HasRelayPacketType;

#[cgp_component {
  provider: RelayPacketFilter,
  context: Relay,
}]
#[async_trait]
pub trait CanFilterRelayPackets: HasRelayPacketType {
    async fn should_relay_packet(&self, packet: &Self::Packet) -> Result<bool, Self::Error>;
}
