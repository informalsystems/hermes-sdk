use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ClientIdOf;
use hermes_relayer_components::multi::traits::birelay_at::{BiRelayAt, HasBiRelayTypeAt};
use hermes_relayer_components::multi::traits::chain_at::ChainAt;
use hermes_relayer_components::multi::traits::relay_at::HasRelayTypeAt;
use hermes_relayer_components::multi::types::index::Twindex;

#[derive_component(BiRelaySetupComponent, BiRelaySetup<Setup>)]
#[async_trait]
pub trait CanSetupBiRelay<const A: usize, const B: usize>:
    HasBiRelayTypeAt<A, B> + HasRelayTypeAt<A, B> + HasRelayTypeAt<B, A> + HasErrorType
where
    ChainAt<Self, A>: HasIbcChainTypes<ChainAt<Self, B>>,
    ChainAt<Self, B>: HasIbcChainTypes<ChainAt<Self, A>>,
{
    async fn setup_birelay(
        &self,
        index: Twindex<A, B>,
        chain_a: &ChainAt<Self, A>,
        chain_b: &ChainAt<Self, B>,
        client_id_a: &ClientIdOf<ChainAt<Self, A>, ChainAt<Self, B>>,
        client_id_b: &ClientIdOf<ChainAt<Self, B>, ChainAt<Self, A>>,
    ) -> Result<BiRelayAt<Self, A, B>, Self::Error>;
}
