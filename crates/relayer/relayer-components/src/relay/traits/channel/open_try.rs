use cgp::prelude::*;

use crate::relay::traits::HasRelayChains;
use crate::relay::types::{DstChannelId, DstPortId, SrcChannelId, SrcPortId};

#[cgp_component {
  provider: ChannelOpenTryRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanRelayChannelOpenTry: HasRelayChains {
    async fn relay_channel_open_try(
        &self,
        dst_port_id: &DstPortId<Self>,
        src_port_id: &SrcPortId<Self>,
        src_channel_id: &SrcChannelId<Self>,
    ) -> Result<DstChannelId<Self>, Self::Error>;
}
