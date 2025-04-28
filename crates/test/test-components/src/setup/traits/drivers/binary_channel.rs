use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_relayer_components::chain::traits::{
    HasChannelIdType, HasConnectionIdType, HasPortIdType,
};
use hermes_relayer_components::chain::types::aliases::{ChannelIdOf, ConnectionIdOf, PortIdOf};
use hermes_relayer_components::multi::traits::birelay_at::{BiRelayAt, HasBiRelayTypeAt};
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};
use hermes_relayer_components::multi::traits::relay_at::HasRelayTypeAt;

use crate::driver::traits::{ChainDriverAt, HasChainDriverTypeAt};
use crate::setup::traits::HasTestDriverType;

#[cgp_component {
    provider: BinaryChannelDriverBuilder,
    context: Setup,
}]
#[async_trait]
pub trait CanBuildTestDriverWithBinaryChannel:
    HasChainTypeAt<
        Index<0>,
        Chain: HasConnectionIdType<ChainAt<Self, Index<1>>>
                   + HasChannelIdType<ChainAt<Self, Index<1>>>
                   + HasPortIdType<ChainAt<Self, Index<1>>>,
    > + HasChainTypeAt<
        Index<1>,
        Chain: HasConnectionIdType<ChainAt<Self, Index<0>>>
                   + HasChannelIdType<ChainAt<Self, Index<0>>>
                   + HasPortIdType<ChainAt<Self, Index<0>>>,
    > + HasBiRelayTypeAt<Index<0>, Index<1>>
    + HasRelayTypeAt<Index<0>, Index<1>>
    + HasRelayTypeAt<Index<1>, Index<0>>
    + HasChainDriverTypeAt<Index<0>>
    + HasChainDriverTypeAt<Index<1>>
    + HasTestDriverType
    + HasAsyncErrorType
{
    async fn build_driver_with_binary_channel(
        &self,
        birelay: BiRelayAt<Self, Index<0>, Index<1>>,
        chain_driver_a: ChainDriverAt<Self, Index<0>>,
        chain_driver_b: ChainDriverAt<Self, Index<1>>,
        connection_id_a: ConnectionIdOf<ChainAt<Self, Index<0>>, ChainAt<Self, Index<1>>>,
        connection_id_b: ConnectionIdOf<ChainAt<Self, Index<1>>, ChainAt<Self, Index<0>>>,
        channel_id_a: ChannelIdOf<ChainAt<Self, Index<0>>, ChainAt<Self, Index<1>>>,
        channel_id_b: ChannelIdOf<ChainAt<Self, Index<1>>, ChainAt<Self, Index<0>>>,
        port_id_a: PortIdOf<ChainAt<Self, Index<0>>, ChainAt<Self, Index<1>>>,
        port_id_b: PortIdOf<ChainAt<Self, Index<1>>, ChainAt<Self, Index<0>>>,
    ) -> Result<Self::TestDriver, Self::Error>;
}
