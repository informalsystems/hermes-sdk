use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ClientIdOf;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[derive_component(ClientSetupComponent, ClientSetup<Setup>)]
#[async_trait]
pub trait CanSetupClients<A: Async, B: Async>:
    HasChainTypeAt<A> + HasChainTypeAt<B> + HasErrorType
where
    ChainAt<Self, A>: HasIbcChainTypes<ChainAt<Self, B>>,
    ChainAt<Self, B>: HasIbcChainTypes<ChainAt<Self, A>>,
{
    async fn setup_clients(
        &self,
        chain_a: &ChainAt<Self, A>,
        chain_b: &ChainAt<Self, B>,
    ) -> Result<
        (
            ClientIdOf<ChainAt<Self, A>, ChainAt<Self, B>>,
            ClientIdOf<ChainAt<Self, B>, ChainAt<Self, A>>,
        ),
        Self::Error,
    >;
}
