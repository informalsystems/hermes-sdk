use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_relayer_components::chain::types::aliases::ClientIdOf;
use hermes_relayer_components::multi::traits::chain_at::ChainAt;
use hermes_relayer_components::multi::traits::relay_at::{HasBoundedRelayTypeAt, RelayAt};

#[cgp_component {
  name: RelaySetupComponent,
  provider: RelaySetup,
  context: Setup,
}]
#[async_trait]
pub trait CanSetupRelays<A: Async, B: Async>:
    HasBoundedRelayTypeAt<A, B> + HasBoundedRelayTypeAt<B, A> + HasErrorType
{
    async fn setup_relays(
        &self,
        _index: PhantomData<(A, B)>,
        chain_a: &ChainAt<Self, A>,
        chain_b: &ChainAt<Self, B>,
        client_id_a: &ClientIdOf<ChainAt<Self, A>, ChainAt<Self, B>>,
        client_id_b: &ClientIdOf<ChainAt<Self, B>, ChainAt<Self, A>>,
    ) -> Result<(RelayAt<Self, A, B>, RelayAt<Self, B, A>), Self::Error>;
}
