use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_relayer_components::chain::types::aliases::ChainId;
use ibc_test_components::bootstrap::traits::types::chain::HasChainType;

#[derive_component(ChainIdGeneratorComponent, ChainIdGenerator<Bootstrap>)]
#[async_trait]
pub trait CanGenerateChainId: HasChainType
where
    Self::Chain: HasChainIdType,
{
    async fn generate_chain_id(&self, chain_id_prefix: &str) -> ChainId<Self::Chain>;
}
