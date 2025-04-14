use cgp::prelude::*;
use hermes_cosmos_chain_components::impls::types::config::CosmosChainConfig;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_test_components::chain::traits::{HasWalletType, WalletOf};
use hermes_test_components::chain_driver::traits::HasChainType;

/**
   Capability for the bootstrap context to build a Hermes v1 relayer chain config.
*/
#[cgp_component {
  provider: RelayerChainConfigBuilder,
  context: Bootstrap,
}]
#[async_trait]
pub trait CanBuildRelayerChainConfig:
    HasChainNodeConfigType + HasChainGenesisConfigType + HasChainType + HasAsyncErrorType
where
    Self::Chain: HasWalletType,
{
    async fn build_relayer_chain_config(
        &self,
        chain_node_config: &Self::ChainNodeConfig,
        chain_genesis_config: &Self::ChainGenesisConfig,
        relayer_wallet: &WalletOf<Self::Chain>,
    ) -> Result<CosmosChainConfig, Self::Error>;
}
