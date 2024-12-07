use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainIdOf;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

#[cgp_component {
  name: ChainIdGeneratorComponent,
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
