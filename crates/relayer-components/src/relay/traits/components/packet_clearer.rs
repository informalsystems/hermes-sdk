use async_trait::async_trait;
use cgp_macros::derive_component;

use crate::chain::types::aliases::{ChannelId, PortId};
use crate::relay::traits::chains::HasRelayChains;
use crate::std_prelude::*;

#[derive_component(PacketClearerComponent, PacketClearer<Relay>)]
#[async_trait]
pub trait CanClearPackets: HasRelayChains {
    async fn clear_packets(
        &self,
        src_channel_id: &ChannelId<Self::SrcChain, Self::DstChain>,
        src_port_id: &PortId<Self::SrcChain, Self::DstChain>,
        dst_channel_id: &ChannelId<Self::DstChain, Self::SrcChain>,
        dst_port_id: &PortId<Self::DstChain, Self::SrcChain>,
    ) -> Result<(), Self::Error>;
}
