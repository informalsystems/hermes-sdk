use cgp_core::{async_trait, derive_component, HasErrorType};

use crate::build::traits::birelay::HasBiRelayType;
use crate::build::types::aliases::{RelayAToB, RelayBToA};
use crate::std_prelude::*;

#[derive_component(BiRelayFromRelayBuilderComponent, BiRelayFromRelayBuilder<Build>)]
#[async_trait]
pub trait CanBuildBiRelayFromRelays: HasBiRelayType + HasErrorType {
    async fn build_birelay_from_relays(
        &self,
        relay_a_to_b: RelayAToB<Self>,
        relay_b_to_a: RelayBToA<Self>,
    ) -> Result<Self::BiRelay, Self::Error>;
}
