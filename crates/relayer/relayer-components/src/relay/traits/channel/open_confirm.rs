use hermes_prelude::*;

use crate::relay::traits::HasRelayChains;
use crate::relay::types::{DstChannelId, DstPortId, SrcChannelId, SrcPortId};

#[cgp_component {
  provider: ChannelOpenConfirmRelayer,
  context: Relay,
}]
#[async_trait]
pub trait CanRelayChannelOpenConfirm: HasRelayChains {
    async fn relay_channel_open_confirm(
        &self,
        dst_port: &DstPortId<Self>,
        dst_channel_id: &DstChannelId<Self>,
        src_port_id: &SrcPortId<Self>,
        src_channel_id: &SrcChannelId<Self>,
    ) -> Result<(), Self::Error>;
}
