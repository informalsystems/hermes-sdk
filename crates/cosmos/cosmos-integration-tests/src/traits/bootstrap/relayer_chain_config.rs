use cgp::prelude::*;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_test_components::chain::traits::types::wallet::{HasWalletType, WalletOf};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use ibc_relayer::chain::cosmos::config::CosmosSdkConfig;

/**
   Capability for the bootstrap context to build a Hermes v1 relayer chain config.
*/
#[derive_component(RelayerChainConfigBuilderComponent, RelayerChainConfigBuilder<Bootstrap>)]
pub trait CanBuildRelayerChainConfig: HasChainNodeConfigType + HasChainType + HasErrorType
where
    Self::Chain: HasWalletType,
{
    fn build_relayer_chain_config(
        &self,
        chain_node_config: &Self::ChainNodeConfig,
        relayer_wallet: &WalletOf<Self::Chain>,
    ) -> Result<CosmosSdkConfig, Self::Error>;
}
