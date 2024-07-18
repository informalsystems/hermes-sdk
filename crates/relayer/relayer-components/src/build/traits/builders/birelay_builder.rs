use cgp_core::prelude::*;

use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::chain::types::aliases::ChainIdOf;
use crate::chain::types::aliases::ClientIdOf;
use crate::multi::traits::birelay_at::HasBiRelayTypeAt;
use crate::multi::traits::chain_at::ChainTypeAt;
use crate::multi::traits::chain_at::HasChainTypeAt;

#[derive_component(BiRelayBuilderComponent, BiRelayBuilder<Build>)]
#[async_trait]
pub trait CanBuildBiRelay<const A: usize, const B: usize>:
    HasBiRelayTypeAt<A, B>
    + HasChainTypeAt<A, Chain: HasChainIdType>
    + HasChainTypeAt<B, Chain: HasChainIdType>
    + HasErrorType
{
    async fn build_birelay(
        &self,
        chain_id_a: &ChainIdOf<ChainTypeAt<Self, A>>,
        chain_id_b: &ChainIdOf<ChainTypeAt<Self, B>>,
        client_id_a: &ClientIdOf<ChainTypeAt<Self, A>, ChainTypeAt<Self, B>>,
        client_id_b: &ClientIdOf<ChainTypeAt<Self, A>, ChainTypeAt<Self, B>>,
    ) -> Result<Self::BiRelay, Self::Error>;
}
