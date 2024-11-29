use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::{ChannelIdOf, ConnectionIdOf, PortIdOf};
use hermes_relayer_components::multi::traits::birelay_at::{BiRelayAt, HasBiRelayTypeAt};
use hermes_relayer_components::multi::traits::chain_at::ChainAt;
use hermes_relayer_components::multi::traits::relay_at::HasBoundedRelayTypeAt;

#[derive_component(ChannelSetupComponent, ChannelSetup<Setup>)]
#[async_trait]
pub trait CanSetupChannel<const A: usize, const B: usize>:
    HasBiRelayTypeAt<A, B> + HasBoundedRelayTypeAt<A, B> + HasBoundedRelayTypeAt<B, A> + HasErrorType
where
    ChainAt<Self, A>: HasIbcChainTypes<ChainAt<Self, B>>,
    ChainAt<Self, B>: HasIbcChainTypes<ChainAt<Self, A>>,
{
    async fn setup_channel(
        &self,
        birelay: &BiRelayAt<Self, A, B>,
        connection_id_a: &ConnectionIdOf<ChainAt<Self, A>, ChainAt<Self, B>>,
        connection_id_b: &ConnectionIdOf<ChainAt<Self, B>, ChainAt<Self, A>>,
    ) -> Result<
        (
            ChannelIdOf<ChainAt<Self, A>, ChainAt<Self, B>>,
            ChannelIdOf<ChainAt<Self, B>, ChainAt<Self, A>>,
            PortIdOf<ChainAt<Self, A>, ChainAt<Self, B>>,
            PortIdOf<ChainAt<Self, B>, ChainAt<Self, A>>,
        ),
        Self::Error,
    >;
}
