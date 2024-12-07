use cgp::prelude::*;

use crate::chain::traits::types::channel::{HasInitChannelOptionsType, InitChannelOptions};
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::types::aliases::{DstPortId, SrcChannelId, SrcPortId};

#[cgp_component {
  name: ChannelInitializerComponent,
  provider: ChannelInitializer,
  context: Relay,
}]
#[async_trait]
pub trait CanInitChannel: HasRelayChains
where
    Self::SrcChain: HasInitChannelOptionsType<Self::DstChain>,
{
    async fn init_channel(
        &self,
        src_port_id: &SrcPortId<Self>,
        dst_port_id: &DstPortId<Self>,
        init_channel_options: &InitChannelOptions<Self::SrcChain, Self::DstChain>,
    ) -> Result<SrcChannelId<Self>, Self::Error>;
}
