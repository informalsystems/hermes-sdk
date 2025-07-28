use core::time::Duration;

use hermes_chain_components::traits::HasClientIdType;
use hermes_prelude::*;

use crate::chain::traits::HasChainIdType;
use crate::multi::traits::birelay_at::HasBiRelayTypeAt;
use crate::multi::traits::chain_at::{ChainAt, ChainIdAt, HasChainTypeAt};
use crate::multi::traits::relay_at::ClientIdAt;

#[cgp_component {
  provider: BiRelayBuilder,
  context: Build,
}]
#[async_trait]
pub trait CanBuildBiRelay<A, B>:
    HasBiRelayTypeAt<A, B>
    + HasChainTypeAt<A, Chain: HasChainIdType + HasClientIdType<ChainAt<Self, B>>>
    + HasChainTypeAt<B, Chain: HasChainIdType + HasClientIdType<ChainAt<Self, A>>>
    + HasAsyncErrorType
where
    ChainAt<Self, A>: HasClientIdType<ChainAt<Self, B>>,
    ChainAt<Self, B>: HasClientIdType<ChainAt<Self, A>>,
{
    async fn build_birelay(
        &self,
        chain_id_a: &ChainIdAt<Self, A>,
        chain_id_b: &ChainIdAt<Self, B>,
        client_id_a: &ClientIdAt<Self, A, B>,
        client_id_b: &ClientIdAt<Self, B, A>,
        refresh_rate_a_to_b: Option<Duration>,
        refresh_rate_b_to_a: Option<Duration>,
    ) -> Result<Self::BiRelay, Self::Error>;
}
