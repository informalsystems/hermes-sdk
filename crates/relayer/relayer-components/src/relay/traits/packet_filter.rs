use cgp::prelude::*;

use crate::relay::traits::chains::{HasRelayChainTypes, PacketOf};

#[cgp_component {
  provider: PacketFilter,
  context: Relay,
}]
#[async_trait]
pub trait CanFilterPackets: HasRelayChainTypes {
    async fn should_relay_packet(&self, packet: &PacketOf<Self>) -> Result<bool, Self::Error>;
}
