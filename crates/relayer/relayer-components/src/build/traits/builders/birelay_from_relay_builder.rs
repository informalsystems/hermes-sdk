use cgp_core::prelude::*;

use crate::multi::traits::birelay_at::HasBiRelayTypeAt;
use crate::multi::traits::relay_at::RelayAt;

#[derive_component(BiRelayFromRelayBuilderComponent, BiRelayFromRelayBuilder<Build>)]
#[async_trait]
pub trait CanBuildBiRelayFromRelays<const A: usize, const B: usize>:
    HasBiRelayTypeAt<A, B> + HasErrorType
{
    async fn build_birelay_from_relays(
        &self,
        relay_a_to_b: RelayAt<Self, A, B>,
        relay_b_to_a: RelayAt<Self, B, A>,
    ) -> Result<Self::BiRelay, Self::Error>;
}
