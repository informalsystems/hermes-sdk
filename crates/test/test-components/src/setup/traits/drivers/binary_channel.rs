use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::{ChannelIdOf, ConnectionIdOf, PortIdOf};
use hermes_relayer_components::multi::traits::birelay_at::{BiRelayAt, HasBiRelayTypeAt};
use hermes_relayer_components::multi::traits::chain_at::ChainAt;
use hermes_relayer_components::multi::traits::relay_at::HasRelayTypeAt;

use crate::driver::traits::types::chain_driver_at::{ChainDriverTypeAt, HasChainDriverTypeAt};
use crate::setup::traits::driver::HasTestDriverType;

#[derive_component(BinaryChannelDriverBuilderComponent, BinaryChannelDriverBuilder<Setup>)]
#[async_trait]
pub trait CanBuildTestDriverWithBinaryChannel:
    HasBiRelayTypeAt<0, 1>
    + HasRelayTypeAt<0, 1>
    + HasRelayTypeAt<1, 0>
    + HasChainDriverTypeAt<0>
    + HasChainDriverTypeAt<1>
    + HasTestDriverType
    + HasErrorType
where
    ChainAt<Self, 0>: HasIbcChainTypes<ChainAt<Self, 1>>,
    ChainAt<Self, 1>: HasIbcChainTypes<ChainAt<Self, 0>>,
{
    async fn build_driver_with_binary_channel(
        &self,
        birelay: BiRelayAt<Self, 0, 1>,
        chain_driver_a: ChainDriverTypeAt<Self, 0>,
        chain_driver_b: ChainDriverTypeAt<Self, 1>,
        connection_id_a: ConnectionIdOf<ChainAt<Self, 0>, ChainAt<Self, 1>>,
        connection_id_b: ConnectionIdOf<ChainAt<Self, 1>, ChainAt<Self, 0>>,
        channel_id_a: ChannelIdOf<ChainAt<Self, 0>, ChainAt<Self, 1>>,
        channel_id_b: ChannelIdOf<ChainAt<Self, 1>, ChainAt<Self, 0>>,
        port_id_a: PortIdOf<ChainAt<Self, 0>, ChainAt<Self, 1>>,
        port_id_b: PortIdOf<ChainAt<Self, 1>, ChainAt<Self, 0>>,
    ) -> Result<Self::TestDriver, Self::Error>;
}
