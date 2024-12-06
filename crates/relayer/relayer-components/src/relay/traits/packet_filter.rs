use cgp::prelude::*;

use crate::relay::traits::chains::{HasRelayChainTypes, PacketOf};

#[derive_component(PacketFilterComponent, PacketFilter<Relay>)]
#[async_trait]
pub trait CanFilterPackets: HasRelayChainTypes {
    async fn should_relay_packet(&self, packet: &PacketOf<Self>) -> Result<bool, Self::Error>;
}
