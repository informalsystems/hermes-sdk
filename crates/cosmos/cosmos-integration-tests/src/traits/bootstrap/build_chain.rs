use cgp_core::prelude::*;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::chain_driver::traits::types::wallet::{HasWalletType, WalletOf};
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

#[derive_component(ChainBuilderWithNodeConfigComponent, ChainBuilderWithNodeConfig<Bootstrap>)]
#[async_trait]
pub trait CanBuildChainWithNodeConfig:
    HasChainType + HasChainNodeConfigType + HasChainDriverType + HasErrorType
where
    Self::ChainDriver: HasWalletType,
{
    async fn build_chain_with_node_config(
        &self,
        chain_node_config: &Self::ChainNodeConfig,
        relayer_wallet: &WalletOf<Self::ChainDriver>,
    ) -> Result<Self::Chain, Self::Error>;
}
