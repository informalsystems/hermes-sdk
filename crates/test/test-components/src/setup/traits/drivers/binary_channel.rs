use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::{ChannelIdOf, ConnectionIdOf, PortIdOf};

use crate::driver::traits::types::birelay_at::{BiRelayTypeAt, HasBiRelayTypeAt};
use crate::driver::traits::types::chain_at::ChainTypeAt;
use crate::driver::traits::types::chain_driver_at::{ChainDriverTypeAt, HasChainDriverTypeAt};
use crate::setup::traits::driver::HasTestDriverType;

#[derive_component(BinaryChannelDriverBuilderComponent, BinaryChannelDriverBuilder<Setup>)]
#[async_trait]
pub trait CanBuildTestDriverWithBinaryChannel:
    HasBiRelayTypeAt<0, 1>
    + HasChainDriverTypeAt<0>
    + HasChainDriverTypeAt<1>
    + HasTestDriverType
    + HasErrorType
where
    ChainTypeAt<Self, 0>: HasIbcChainTypes<ChainTypeAt<Self, 1>>,
    ChainTypeAt<Self, 1>: HasIbcChainTypes<ChainTypeAt<Self, 0>>,
{
    async fn build_driver_with_binary_channel(
        &self,
        birelay: BiRelayTypeAt<Self, 0, 1>,
        chain_driver_a: ChainDriverTypeAt<Self, 0>,
        chain_driver_b: ChainDriverTypeAt<Self, 1>,
        connection_id_a: ConnectionIdOf<ChainTypeAt<Self, 0>, ChainTypeAt<Self, 1>>,
        connection_id_b: ConnectionIdOf<ChainTypeAt<Self, 1>, ChainTypeAt<Self, 0>>,
        channel_id_a: ChannelIdOf<ChainTypeAt<Self, 0>, ChainTypeAt<Self, 1>>,
        channel_id_b: ChannelIdOf<ChainTypeAt<Self, 1>, ChainTypeAt<Self, 0>>,
        port_id_a: PortIdOf<ChainTypeAt<Self, 0>, ChainTypeAt<Self, 1>>,
        port_id_b: PortIdOf<ChainTypeAt<Self, 1>, ChainTypeAt<Self, 0>>,
    ) -> Result<Self::TestDriver, Self::Error>;
}
