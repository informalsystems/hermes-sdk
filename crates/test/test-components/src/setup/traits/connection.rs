use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ConnectionIdOf;
use hermes_relayer_components::multi::traits::birelay_at::{BiRelayAt, HasBiRelayTypeAt};
use hermes_relayer_components::multi::traits::chain_at::ChainAt;

#[derive_component(ConnectionSetupComponent, ConnectionSetup<Setup>)]
#[async_trait]
pub trait CanSetupConnection<const A: usize, const B: usize>:
    HasBiRelayTypeAt<A, B> + HasErrorType
where
    ChainAt<Self, A>: HasIbcChainTypes<ChainAt<Self, B>>,
    ChainAt<Self, B>: HasIbcChainTypes<ChainAt<Self, A>>,
{
    async fn setup_connection(
        &self,
        birelay: &BiRelayAt<Self, A, B>,
    ) -> Result<
        (
            ConnectionIdOf<ChainAt<Self, A>, ChainAt<Self, B>>,
            ConnectionIdOf<ChainAt<Self, B>, ChainAt<Self, A>>,
        ),
        Self::Error,
    >;
}
