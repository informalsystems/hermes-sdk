use cgp::prelude::*;

use crate::relay::traits::chains::{HasRelayChainTypes, PacketOf};

#[derive_component(PacketRelayerComponent, PacketRelayer<Relay>)]
#[async_trait]
pub trait CanRelayPacket: HasRelayChainTypes {
    async fn relay_packet(&self, packet: &PacketOf<Self>) -> Result<(), Self::Error>;
}
