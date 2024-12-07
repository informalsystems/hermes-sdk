use cgp::prelude::*;

use crate::chain::types::aliases::{ChannelIdOf, PortIdOf};
use crate::relay::traits::chains::HasRelayChains;

#[cgp_component {
  name: PacketClearerComponent,
  provider: PacketClearer,
  context: Relay,
}]
#[async_trait]
pub trait CanClearPackets: HasRelayChains {
    async fn clear_packets(
        &self,
        src_channel_id: &ChannelIdOf<Self::SrcChain, Self::DstChain>,
        src_port_id: &PortIdOf<Self::SrcChain, Self::DstChain>,
        dst_channel_id: &ChannelIdOf<Self::DstChain, Self::SrcChain>,
        dst_port_id: &PortIdOf<Self::DstChain, Self::SrcChain>,
    ) -> Result<(), Self::Error>;
}
