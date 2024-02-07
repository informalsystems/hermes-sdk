use cgp_core::prelude::*;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_test_components::chain_driver::traits::types::wallet::{HasWalletType, WalletOf};
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use ibc_relayer::config::ChainConfig;

/**
   Capability for the bootstrap context to build a Hermes v1 relayer chain config.
*/
#[derive_component(RelayerChainConfigBuilderComponent, RelayerChainConfigBuilder<Bootstrap>)]
pub trait CanBuildRelayerChainConfig:
    HasChainNodeConfigType + HasChainDriverType + HasErrorType
where
    Self::ChainDriver: HasWalletType,
{
    fn build_relayer_chain_config(
        &self,
        chain_node_config: &Self::ChainNodeConfig,
        relayer_wallet: &WalletOf<Self::ChainDriver>,
    ) -> Result<ChainConfig, Self::Error>;
}
