use hermes_prelude::*;
use hermes_relayer_components::chain::traits::HasClientIdType;
use hermes_relayer_components::chain::types::aliases::ClientIdOf;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[cgp_component {
  provider: ClientSetup,
  context: Setup,
}]
#[async_trait]
pub trait CanSetupClients<A, B>:
    HasChainTypeAt<A, Chain: HasClientIdType<ChainAt<Self, B>>>
    + HasChainTypeAt<B, Chain: HasClientIdType<ChainAt<Self, A>>>
    + HasAsyncErrorType
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
