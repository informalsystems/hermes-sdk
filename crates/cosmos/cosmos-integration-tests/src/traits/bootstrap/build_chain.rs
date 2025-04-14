use cgp::prelude::*;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_test_components::chain::traits::{HasWalletType, WalletOf};
use hermes_test_components::chain_driver::traits::HasChainType;

#[cgp_component {
  provider: ChainBuilderWithNodeConfig,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanBuildChainWithNodeConfig:
    HasChainType + HasChainNodeConfigType + HasChainGenesisConfigType + HasChainType + HasAsyncErrorType
where
    Self::Chain: HasWalletType,
{
    async fn build_chain_with_node_config(
        &self,
        chain_node_config: &Self::ChainNodeConfig,
        chain_genesis_config: &Self::ChainGenesisConfig,
        relayer_wallet: &WalletOf<Self::Chain>,
    ) -> Result<Self::Chain, Self::Error>;
}
