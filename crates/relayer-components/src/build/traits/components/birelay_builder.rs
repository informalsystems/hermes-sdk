use cgp_core::prelude::*;

use crate::build::traits::birelay::HasBiRelayType;
use crate::build::types::aliases::{ChainA, ChainB, ChainIdA, ChainIdB, ClientIdA, ClientIdB};
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::std_prelude::*;

#[derive_component(BiRelayBuilderComponent, BiRelayBuilder<Build>)]
#[async_trait]
pub trait CanBuildBiRelay: HasBiRelayType + HasErrorType
where
    ChainA<Self>: HasIbcChainTypes<ChainB<Self>>,
    ChainB<Self>: HasIbcChainTypes<ChainA<Self>>,
{
    async fn build_birelay(
        &self,
        chain_id_a: &ChainIdA<Self>,
        chain_id_b: &ChainIdB<Self>,
        client_id_a: &ClientIdA<Self>,
        client_id_b: &ClientIdB<Self>,
    ) -> Result<Self::BiRelay, Self::Error>;
}
