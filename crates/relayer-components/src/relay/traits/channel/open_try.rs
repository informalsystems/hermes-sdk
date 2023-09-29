use cgp_async::async_generic_trait;
use cgp_macros::derive_component;

use crate::relay::traits::chains::HasRelayChains;
use crate::relay::types::aliases::{DstChannelId, DstPortId, SrcChannelId, SrcPortId};
use crate::std_prelude::*;

#[derive_component(ChannelOpenTryRelayerComponent, ChannelOpenTryRelayer<Relay>)]
#[async_generic_trait]
pub trait CanRelayChannelOpenTry: HasRelayChains {
    async fn relay_channel_open_try(
        &self,
        dst_port_id: &DstPortId<Self>,
        src_port_id: &SrcPortId<Self>,
        src_channel_id: &SrcChannelId<Self>,
    ) -> Result<DstChannelId<Self>, Self::Error>;
}
