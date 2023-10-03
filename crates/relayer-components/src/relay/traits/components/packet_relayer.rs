use cgp_core::{async_trait, derive_component};

use crate::relay::traits::chains::HasRelayChains;
use crate::std_prelude::*;

#[derive_component(PacketRelayerComponent, PacketRelayer<Relay>)]
#[async_trait]
pub trait CanRelayPacket: HasRelayChains {
    async fn relay_packet(&self, packet: &Self::Packet) -> Result<(), Self::Error>;
}
