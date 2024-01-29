use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::types::aliases::ClientIdOf;

use crate::driver::traits::types::birelay_at::{BiRelayTypeAt, HasBiRelayTypeAt};
use crate::driver::traits::types::chain_at::ChainTypeAt;
use crate::types::index::Twindex;

#[derive_component(BiRelaySetupComponent, BiRelaySetup<Setup>)]
#[async_trait]
pub trait CanSetupBiRelay<const A: usize, const B: usize>:
    HasBiRelayTypeAt<A, B> + HasErrorType
where
    ChainTypeAt<Self, A>: HasIbcChainTypes<ChainTypeAt<Self, B>>,
    ChainTypeAt<Self, B>: HasIbcChainTypes<ChainTypeAt<Self, A>>,
{
    async fn setup_birelay(
        &self,
        index: Twindex<A, B>,
        chain_a: &ChainTypeAt<Self, A>,
        chain_b: &ChainTypeAt<Self, B>,
        client_id_a: &ClientIdOf<ChainTypeAt<Self, A>, ChainTypeAt<Self, B>>,
        client_id_b: &ClientIdOf<ChainTypeAt<Self, B>, ChainTypeAt<Self, A>>,
    ) -> Result<BiRelayTypeAt<Self, A, B>, Self::Error>;
}
