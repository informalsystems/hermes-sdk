use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ClientIdOf;
use hermes_relayer_components::multi::traits::chain_at::{ChainTypeAt, HasChainTypeAt};

#[derive_component(ClientSetupComponent, ClientSetup<Setup>)]
#[async_trait]
pub trait CanSetupClients<const A: usize, const B: usize>:
    HasChainTypeAt<A> + HasChainTypeAt<B> + HasErrorType
where
    ChainTypeAt<Self, A>: HasIbcChainTypes<ChainTypeAt<Self, B>>,
    ChainTypeAt<Self, B>: HasIbcChainTypes<ChainTypeAt<Self, A>>,
{
    async fn setup_clients(
        &self,
        chain_a: &ChainTypeAt<Self, A>,
        chain_b: &ChainTypeAt<Self, B>,
    ) -> Result<
        (
            ClientIdOf<ChainTypeAt<Self, A>, ChainTypeAt<Self, B>>,
            ClientIdOf<ChainTypeAt<Self, B>, ChainTypeAt<Self, A>>,
        ),
        Self::Error,
    >;
}
