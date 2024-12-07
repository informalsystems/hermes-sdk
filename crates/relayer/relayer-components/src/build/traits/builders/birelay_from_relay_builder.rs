use cgp::prelude::*;

use crate::multi::traits::birelay_at::HasBiRelayTypeAt;
use crate::multi::traits::relay_at::{HasRelayTypeAt, RelayAt};

#[cgp_component {
  name: BiRelayFromRelayBuilderComponent,
  provider: BiRelayFromRelayBuilder,
  context: Build,
}]
#[async_trait]
pub trait CanBuildBiRelayFromRelays<A, B>:
    HasBiRelayTypeAt<A, B> + HasRelayTypeAt<A, B> + HasRelayTypeAt<B, A> + HasErrorType
{
    async fn build_birelay_from_relays(
        &self,
        relay_a_to_b: RelayAt<Self, A, B>,
        relay_b_to_a: RelayAt<Self, B, A>,
    ) -> Result<Self::BiRelay, Self::Error>;
}
