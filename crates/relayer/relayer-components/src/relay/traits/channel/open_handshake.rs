use cgp::prelude::*;

use crate::relay::traits::HasRelayChains;
use crate::relay::types::{DstChannelId, DstPortId, SrcChannelId, SrcPortId};

#[cgp_component {
  provider: ChannelOpenHandshakeRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanRelayChannelOpenHandshake: HasRelayChains {
    async fn relay_channel_open_handshake(
        &self,
        src_channel_id: &SrcChannelId<Self>,
        src_port_id: &SrcPortId<Self>,
        dst_port_id: &DstPortId<Self>,
    ) -> Result<DstChannelId<Self>, Self::Error>;
}
