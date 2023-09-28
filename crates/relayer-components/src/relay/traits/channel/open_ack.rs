use async_trait::async_trait;
use cgp_macros::derive_component;

use crate::relay::traits::chains::HasRelayChains;
use crate::relay::types::aliases::{DstChannelId, DstPortId, SrcChannelId, SrcPortId};
use crate::std_prelude::*;

#[derive_component(ChannelOpenAckRelayerComponent, ChannelOpenAckRelayer<Relay>)]
#[async_trait]
pub trait CanRelayChannelOpenAck: HasRelayChains {
    async fn relay_channel_open_ack(
        &self,
        src_port_id: &SrcPortId<Self>,
        src_channel_id: &SrcChannelId<Self>,
        dst_port_id: &DstPortId<Self>,
        dst_channel_id: &DstChannelId<Self>,
    ) -> Result<(), Self::Error>;
}
