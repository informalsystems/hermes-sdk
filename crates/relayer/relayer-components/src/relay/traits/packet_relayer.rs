use cgp::prelude::*;

use crate::relay::traits::chains::{HasRelayChainTypes, PacketOf};

#[cgp_component {
  name: PacketRelayerComponent,
  provider: PacketRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanRelayPacket: HasRelayChainTypes {
    async fn relay_packet(&self, packet: &PacketOf<Self>) -> Result<(), Self::Error>;
}
