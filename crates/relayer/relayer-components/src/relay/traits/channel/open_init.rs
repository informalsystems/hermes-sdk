use hermes_prelude::*;

use crate::chain::traits::{HasInitChannelOptionsType, InitChannelOptions};
use crate::relay::traits::HasRelayChains;
use crate::relay::types::{DstPortId, SrcChannelId, SrcPortId};

#[cgp_component {
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
