use core::marker::PhantomData;
use core::time::Duration;

use hermes_prelude::*;
use hermes_relayer_components::chain::traits::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ClientIdOf;
use hermes_relayer_components::multi::traits::birelay_at::{BiRelayAt, HasBiRelayTypeAt};
use hermes_relayer_components::multi::traits::chain_at::ChainAt;
use hermes_relayer_components::multi::traits::relay_at::HasBoundedRelayTypeAt;

#[cgp_component {
  provider: BiRelaySetup,
  context: Setup,
}]
#[async_trait]
pub trait CanSetupBiRelay<A: Async, B: Async>:
    HasBiRelayTypeAt<A, B>
    + HasBoundedRelayTypeAt<A, B>
    + HasBoundedRelayTypeAt<B, A>
    + HasAsyncErrorType
where
    ChainAt<Self, A>: HasIbcChainTypes<ChainAt<Self, B>>,
    ChainAt<Self, B>: HasIbcChainTypes<ChainAt<Self, A>>,
{
    async fn setup_birelay(
        &self,
        _index: PhantomData<(A, B)>,
        chain_a: &ChainAt<Self, A>,
        chain_b: &ChainAt<Self, B>,
        client_id_a: &ClientIdOf<ChainAt<Self, A>, ChainAt<Self, B>>,
        client_id_b: &ClientIdOf<ChainAt<Self, B>, ChainAt<Self, A>>,
        refresh_rate_a: Option<Duration>,
        refresh_rate_b: Option<Duration>,
    ) -> Result<BiRelayAt<Self, A, B>, Self::Error>;
}
