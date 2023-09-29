use cgp_async::async_generic_trait;
use cgp_macros::derive_component;

use crate::relay::traits::chains::HasRelayChains;
use crate::relay::types::aliases::{DstChannelId, DstPortId, SrcChannelId, SrcPortId};
use crate::std_prelude::*;

#[derive_component(ChannelOpenConfirmRelayerComponent, ChannelOpenConfirmRelayer<Relay>)]
#[async_generic_trait]
pub trait CanRelayChannelOpenConfirm: HasRelayChains {
    async fn relay_channel_open_confirm(
        &self,
        dst_port: &DstPortId<Self>,
        dst_channel_id: &DstChannelId<Self>,
        src_port_id: &SrcPortId<Self>,
        src_channel_id: &SrcChannelId<Self>,
    ) -> Result<(), Self::Error>;
}
