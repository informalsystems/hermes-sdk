use cgp::prelude::*;

use crate::relay::traits::chains::{HasRelayChains, PacketOf};

#[derive_component(PacketFilterComponent, PacketFilter<Relay>)]
#[async_trait]
pub trait CanFilterPackets: HasRelayChains {
    async fn should_relay_packet(&self, packet: &PacketOf<Self>) -> Result<bool, Self::Error>;
}
