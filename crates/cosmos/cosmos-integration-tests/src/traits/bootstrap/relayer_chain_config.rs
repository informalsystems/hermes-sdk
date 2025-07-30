use hermes_core::test_components::chain::traits::{HasWalletType, WalletOf};
use hermes_core::test_components::chain_driver::traits::HasChainType;
use hermes_cosmos_core::chain_components::impls::CosmosChainConfig;
use hermes_cosmos_core::test_components::bootstrap::traits::{
    HasChainGenesisConfigType, HasChainNodeConfigType,
};
use hermes_prelude::*;

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
        relayer_wallets: Vec<&WalletOf<Self::Chain>>,
    ) -> Result<CosmosChainConfig, Self::Error>;
}
