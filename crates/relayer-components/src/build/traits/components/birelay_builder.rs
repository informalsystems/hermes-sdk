use cgp_core::prelude::*;
use cgp_core::HasErrorType;

use crate::build::traits::birelay::HasBiRelayType;
use crate::build::types::aliases::{ChainIdA, ChainIdB, ClientIdA, ClientIdB};
use crate::std_prelude::*;

#[derive_component(BiRelayBuilderComponent, BiRelayBuilder<Build>)]
#[async_trait]
pub trait CanBuildBiRelay: HasBiRelayType + HasErrorType {
    async fn build_birelay(
        &self,
        chain_id_a: &ChainIdA<Self>,
        chain_id_b: &ChainIdB<Self>,
        client_id_a: &ClientIdA<Self>,
        client_id_b: &ClientIdB<Self>,
    ) -> Result<Self::BiRelay, Self::Error>;
}
