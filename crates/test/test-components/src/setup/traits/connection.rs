use cgp::prelude::*;
use hermes_relayer_components::chain::types::aliases::ConnectionIdOf;
use hermes_relayer_components::multi::traits::birelay_at::{BiRelayAt, HasBiRelayTypeAt};
use hermes_relayer_components::multi::traits::chain_at::ChainAt;
use hermes_relayer_components::multi::traits::relay_at::HasBoundedRelayTypeAt;

#[cgp_component {
  name: ConnectionSetupComponent,
  provider: ConnectionSetup,
  context: Setup,
}]
#[async_trait]
pub trait CanSetupConnection<A: Async, B: Async>:
    HasBiRelayTypeAt<A, B> + HasBoundedRelayTypeAt<A, B> + HasBoundedRelayTypeAt<B, A> + HasErrorType
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
