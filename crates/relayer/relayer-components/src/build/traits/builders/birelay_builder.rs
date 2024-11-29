use cgp::prelude::*;

use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::multi::traits::birelay_at::HasBiRelayTypeAt;
use crate::multi::traits::chain_at::{ChainIdAt, HasChainTypeAt};
use crate::multi::traits::relay_at::{ClientIdAt, HasRelayTypeAt};

#[derive_component(BiRelayBuilderComponent, BiRelayBuilder<Build>)]
#[async_trait]
pub trait CanBuildBiRelay<const A: usize, const B: usize>:
    HasBiRelayTypeAt<A, B>
    + HasChainTypeAt<A, Chain: HasChainIdType>
    + HasChainTypeAt<B, Chain: HasChainIdType>
    + HasRelayTypeAt<A, B>
    + HasRelayTypeAt<B, A>
    + HasErrorType
{
    async fn build_birelay(
        &self,
        chain_id_a: &ChainIdAt<Self, A>,
        chain_id_b: &ChainIdAt<Self, B>,
        client_id_a: &ClientIdAt<Self, A, B>,
        client_id_b: &ClientIdAt<Self, B, A>,
    ) -> Result<Self::BiRelay, Self::Error>;
}
