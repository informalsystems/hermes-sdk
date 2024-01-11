use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::{ChannelId, ConnectionId};

use crate::driver::traits::types::birelay_at::{BiRelayTypeAt, HasBiRelayTypeAt};
use crate::driver::traits::types::chain_at::ChainTypeAt;
use crate::setup::traits::driver::HasDriverType;

#[derive_component(BinaryChannelDriverBuilderComponent, BinaryChannelDriverBuilder<Setup>)]
#[async_trait]
pub trait CanBuildDriverWithBinaryChannel:
    HasBiRelayTypeAt<0, 1> + HasDriverType + HasErrorType
where
    ChainTypeAt<Self, 0>: HasIbcChainTypes<ChainTypeAt<Self, 1>>,
    ChainTypeAt<Self, 1>: HasIbcChainTypes<ChainTypeAt<Self, 0>>,
{
    async fn build_driver_with_binary_channel(
        &self,
        birelay: BiRelayTypeAt<Self, 0, 1>,
        connection_id_a: ConnectionId<ChainTypeAt<Self, 0>, ChainTypeAt<Self, 1>>,
        connection_id_b: ConnectionId<ChainTypeAt<Self, 1>, ChainTypeAt<Self, 0>>,
        channel_id_a: ChannelId<ChainTypeAt<Self, 0>, ChainTypeAt<Self, 1>>,
        channel_id_b: ChannelId<ChainTypeAt<Self, 1>, ChainTypeAt<Self, 0>>,
    ) -> Result<Self::Driver, Self::Error>;
}
