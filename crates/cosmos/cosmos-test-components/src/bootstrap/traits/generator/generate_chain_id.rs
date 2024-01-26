use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainId;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

#[derive_component(ChainIdGeneratorComponent, ChainIdGenerator<Bootstrap>)]
#[async_trait]
pub trait CanGenerateChainId: HasChainType
where
    Self::Chain: HasChainIdType,
{
    async fn generate_chain_id(&self, chain_id_prefix: &str) -> ChainId<Self::Chain>;
}
