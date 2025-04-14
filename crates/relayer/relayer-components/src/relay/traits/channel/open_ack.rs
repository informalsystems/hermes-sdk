use cgp::prelude::*;

use crate::relay::traits::HasRelayChains;
use crate::relay::types::{DstChannelId, DstPortId, SrcChannelId, SrcPortId};

#[cgp_component {
  provider: ChannelOpenAckRelayer,
  context: Relay,
}]
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
