use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::{ChannelIdOf, ConnectionIdOf, PortIdOf};
use hermes_relayer_components::multi::traits::birelay_at::{BiRelayAt, HasBiRelayTypeAt};
use hermes_relayer_components::multi::traits::chain_at::ChainAt;
use hermes_relayer_components::multi::traits::relay_at::HasBoundedRelayTypeAt;
use hermes_relayer_components::multi::types::index::Index;

use crate::driver::traits::types::chain_driver_at::{ChainDriverTypeAt, HasChainDriverTypeAt};
use crate::setup::traits::driver::HasTestDriverType;

#[cgp_component {
  provider: BinaryChannelDriverBuilder,
  context: Setup,
}]
#[async_trait]
pub trait CanBuildTestDriverWithBinaryChannel:
    HasBiRelayTypeAt<Index<0>, Index<1>>
    + HasBoundedRelayTypeAt<Index<0>, Index<1>>
    + HasBoundedRelayTypeAt<Index<1>, Index<0>>
    + HasChainDriverTypeAt<Index<0>>
    + HasChainDriverTypeAt<Index<1>>
    + HasTestDriverType
    + HasAsyncErrorType
where
    ChainAt<Self, Index<0>>: HasIbcChainTypes<ChainAt<Self, Index<1>>>,
    ChainAt<Self, Index<1>>: HasIbcChainTypes<ChainAt<Self, Index<0>>>,
{
    async fn build_driver_with_binary_channel(
        &self,
        birelay: BiRelayAt<Self, Index<0>, Index<1>>,
        chain_driver_a: ChainDriverTypeAt<Self, Index<0>>,
        chain_driver_b: ChainDriverTypeAt<Self, Index<1>>,
        connection_id_a: ConnectionIdOf<ChainAt<Self, Index<0>>, ChainAt<Self, Index<1>>>,
        connection_id_b: ConnectionIdOf<ChainAt<Self, Index<1>>, ChainAt<Self, Index<0>>>,
        channel_id_a: ChannelIdOf<ChainAt<Self, Index<0>>, ChainAt<Self, Index<1>>>,
        channel_id_b: ChannelIdOf<ChainAt<Self, Index<1>>, ChainAt<Self, Index<0>>>,
        port_id_a: PortIdOf<ChainAt<Self, Index<0>>, ChainAt<Self, Index<1>>>,
        port_id_b: PortIdOf<ChainAt<Self, Index<1>>, ChainAt<Self, Index<0>>>,
    ) -> Result<Self::TestDriver, Self::Error>;
}
