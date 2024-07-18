use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::{ChannelIdOf, ConnectionIdOf, PortIdOf};
use hermes_relayer_components::multi::traits::birelay_at::{BiRelayTypeAt, HasBiRelayTypeAt};
use hermes_relayer_components::multi::traits::chain_at::ChainTypeAt;

#[derive_component(ChannelSetupComponent, ChannelSetup<Setup>)]
#[async_trait]
pub trait CanSetupChannel<const A: usize, const B: usize>:
    HasBiRelayTypeAt<A, B> + HasErrorType
where
    ChainTypeAt<Self, A>: HasIbcChainTypes<ChainTypeAt<Self, B>>,
    ChainTypeAt<Self, B>: HasIbcChainTypes<ChainTypeAt<Self, A>>,
{
    async fn setup_channel(
        &self,
        birelay: &BiRelayTypeAt<Self, A, B>,
        connection_id_a: &ConnectionIdOf<ChainTypeAt<Self, A>, ChainTypeAt<Self, B>>,
        connection_id_b: &ConnectionIdOf<ChainTypeAt<Self, B>, ChainTypeAt<Self, A>>,
    ) -> Result<
        (
            ChannelIdOf<ChainTypeAt<Self, A>, ChainTypeAt<Self, B>>,
            ChannelIdOf<ChainTypeAt<Self, B>, ChainTypeAt<Self, A>>,
            PortIdOf<ChainTypeAt<Self, A>, ChainTypeAt<Self, B>>,
            PortIdOf<ChainTypeAt<Self, B>, ChainTypeAt<Self, A>>,
        ),
        Self::Error,
    >;
}
