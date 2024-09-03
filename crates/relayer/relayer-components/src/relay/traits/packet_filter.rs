use cgp::prelude::*;

use crate::relay::traits::chains::HasRelayChains;

#[derive_component(PacketFilterComponent, PacketFilter<Relay>)]
#[async_trait]
pub trait CanFilterPackets: HasRelayChains {
    async fn should_relay_packet(&self, packet: &Self::Packet) -> Result<bool, Self::Error>;
}
