use cgp::prelude::*;
use hermes_cosmos_chain_components::traits::eip::eip_type::EipQueryType;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_test_components::chain::traits::types::wallet::{HasWalletType, WalletOf};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

#[derive_component(ChainBuilderWithNodeConfigComponent, ChainBuilderWithNodeConfig<Bootstrap>)]
#[async_trait]
pub trait CanBuildChainWithNodeConfig:
    HasChainType + HasChainNodeConfigType + HasChainGenesisConfigType + HasChainType + HasErrorType
where
    Self::Chain: HasWalletType,
{
    async fn build_chain_with_node_config(
        &self,
        chain_node_config: &Self::ChainNodeConfig,
        chain_genesis_config: &Self::ChainGenesisConfig,
        relayer_wallet: &WalletOf<Self::Chain>,
        eip_query_type: EipQueryType,
    ) -> Result<Self::Chain, Self::Error>;
}
