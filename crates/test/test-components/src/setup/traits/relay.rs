use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ClientIdOf;
use hermes_relayer_components::multi::traits::chain_at::ChainAt;
use hermes_relayer_components::multi::traits::relay_at::{HasRelayTypeAt, RelayTypeAt};
use hermes_relayer_components::multi::types::index::Twindex;

#[derive_component(RelaySetupComponent, RelaySetup<Setup>)]
#[async_trait]
pub trait CanSetupRelays<const A: usize, const B: usize>:
    HasRelayTypeAt<A, B> + HasRelayTypeAt<B, A> + HasErrorType
where
    ChainAt<Self, A>: HasIbcChainTypes<ChainAt<Self, B>>,
    ChainAt<Self, B>: HasIbcChainTypes<ChainAt<Self, A>>,
{
    async fn setup_relays(
        &self,
        index: Twindex<A, B>,
        chain_a: &ChainAt<Self, A>,
        chain_b: &ChainAt<Self, B>,
        client_id_a: &ClientIdOf<ChainAt<Self, A>, ChainAt<Self, B>>,
        client_id_b: &ClientIdOf<ChainAt<Self, B>, ChainAt<Self, A>>,
    ) -> Result<(RelayTypeAt<Self, A, B>, RelayTypeAt<Self, B, A>), Self::Error>;
}
