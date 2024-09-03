use cgp::prelude::*;

use crate::relay::traits::chains::HasRelayChains;

#[derive_component(PacketRelayerComponent, PacketRelayer<Relay>)]
#[async_trait]
pub trait CanRelayPacket: HasRelayChains {
    async fn relay_packet(&self, packet: &Self::Packet) -> Result<(), Self::Error>;
}
