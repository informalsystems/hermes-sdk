use cgp::prelude::*;

use crate::relay::traits::chains::{HasRelayChainTypes, PacketOf};

#[cgp_component {
  provider: RelayPacketFilter,
  context: Relay,
}]
#[async_trait]
pub trait CanFilterRelayPackets: HasRelayChainTypes {
    async fn should_relay_packet(&self, packet: &PacketOf<Self>) -> Result<bool, Self::Error>;
}
