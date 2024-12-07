use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasClientIdType;
use hermes_relayer_components::chain::types::aliases::ClientIdOf;
use hermes_relayer_components::multi::traits::chain_at::{ChainAt, HasChainTypeAt};

#[cgp_component {
  name: ClientSetupComponent,
  provider: ClientSetup,
  context: Setup,
}]
#[async_trait]
pub trait CanSetupClients<A: Async, B: Async>:
    HasChainTypeAt<A, Chain: HasClientIdType<ChainAt<Self, B>>>
    + HasChainTypeAt<B, Chain: HasClientIdType<ChainAt<Self, A>>>
    + HasErrorType
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
