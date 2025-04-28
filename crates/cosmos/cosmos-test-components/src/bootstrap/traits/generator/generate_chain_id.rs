use hermes_core::relayer_components::chain::traits::HasChainIdType;
use hermes_core::relayer_components::chain::types::aliases::ChainIdOf;
use hermes_core::test_components::chain_driver::traits::HasChainType;
use hermes_prelude::*;

#[cgp_component {
  provider: ChainIdGenerator,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanGenerateChainId: HasChainType
where
    Self::Chain: HasChainIdType,
{
    async fn generate_chain_id(&self, chain_id_prefix: &str) -> ChainIdOf<Self::Chain>;
}
