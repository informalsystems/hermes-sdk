use cgp_core::CanRaiseError;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_relayer::types::error::Error as CosmosError;
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::chain_driver::types::wallet::CosmosTestWallet;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::chain_driver::traits::types::wallet::HasWalletType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::traits::bootstrap::build_chain::ChainBuilderWithNodeConfig;
use crate::traits::bootstrap::cosmos_builder::HasCosmosBuilder;
use crate::traits::bootstrap::relayer_chain_config::CanBuildRelayerChainConfig;

pub struct BuildCosmosChain;

impl<Bootstrap, ChainDriver> ChainBuilderWithNodeConfig<Bootstrap> for BuildCosmosChain
where
    Bootstrap: HasChainType<Chain = CosmosChain>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + HasChainNodeConfigType
        + CanBuildRelayerChainConfig
        + HasCosmosBuilder
        + CanRaiseError<CosmosError>,
    ChainDriver: HasWalletType<Wallet = CosmosTestWallet>,
{
    async fn build_chain_with_node_config(
        bootstrap: &Bootstrap,
        chain_node_config: &Bootstrap::ChainNodeConfig,
        relayer_wallet: &CosmosTestWallet,
    ) -> Result<CosmosChain, Bootstrap::Error> {
        let relayer_chain_config =
            bootstrap.build_relayer_chain_config(&chain_node_config, &relayer_wallet)?;

        let chain = bootstrap
            .cosmos_builder()
            .build_chain_with_config(
                relayer_chain_config.clone(),
                Some(&relayer_wallet.keypair.clone()),
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(chain)
    }
}
