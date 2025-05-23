use alloc::collections::BTreeMap;

use hermes_core::relayer_components::chain::traits::HasChainIdType;
use hermes_core::runtime_components::traits::{HasChildProcessType, HasFilePathType, HasRuntime};
use hermes_core::test_components::bootstrap::traits::{
    ChainBootstrapper, ChainBootstrapperComponent,
};
use hermes_core::test_components::chain::traits::HasWalletType;
use hermes_core::test_components::chain_driver::traits::HasChainType;
use hermes_core::test_components::driver::traits::HasChainDriverType;
use hermes_prelude::*;

use crate::bootstrap::traits::{
    CanAddWalletToGenesis, CanBuildChainDriver, CanCollectGenesisTransactions, CanGenerateChainId,
    CanGenerateWalletConfigs, CanInitChainData, CanInitChainGenesisConfig, CanInitChainHomeDir,
    CanInitChainNodeConfig, CanStartChainFullNodes,
};

pub struct BootstrapCosmosChain;

#[cgp_provider(ChainBootstrapperComponent)]
impl<Bootstrap, Runtime, Chain, ChainDriver> ChainBootstrapper<Bootstrap> for BootstrapCosmosChain
where
    Bootstrap: HasAsyncErrorType
        + HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + CanGenerateChainId
        + CanInitChainHomeDir
        + CanInitChainData
        + CanInitChainGenesisConfig
        + CanGenerateWalletConfigs
        + CanAddWalletToGenesis
        + CanCollectGenesisTransactions
        + CanInitChainNodeConfig
        + CanStartChainFullNodes
        + CanBuildChainDriver,
    Runtime: HasFilePathType + HasChildProcessType + HasAsyncErrorType,
    Chain: HasChainIdType + HasWalletType,
{
    async fn bootstrap_chain(
        bootstrap: &Bootstrap,
        chain_id_prefix: &str,
    ) -> Result<ChainDriver, Bootstrap::Error> {
        let chain_id = bootstrap.generate_chain_id(chain_id_prefix).await;

        let chain_home_dir = bootstrap.init_chain_home_dir(&chain_id).await?;

        // Run the `init` chain CLI subcommand to initialize the chain data files on the
        // given chain home directory.
        bootstrap
            .init_chain_data(&chain_home_dir, &chain_id)
            .await?;

        // Initialize (or update) the genesis config file on the chain home directory
        let genesis_config = bootstrap.init_genesis_config(&chain_home_dir).await?;

        let wallets = {
            // Generate configuration of wallets that should be added to genesis
            let wallet_configs = bootstrap.generate_wallet_configs(&genesis_config).await?;

            let mut wallets = BTreeMap::new();

            for (key, wallet_config) in wallet_configs {
                let wallet = bootstrap
                    .add_wallet_to_genesis(&chain_home_dir, &chain_id, &wallet_config)
                    .await?;

                wallets.insert(key, wallet);
            }

            wallets
        };

        // Run the collect-gentxs command
        bootstrap
            .collect_genesis_transactions(&chain_home_dir)
            .await?;

        // Initialize (or update) the chain config files that are required for starting
        // the chain full node
        let chain_config = bootstrap
            .init_chain_node_config(&chain_home_dir, &chain_id, &genesis_config)
            .await?;

        // Start the chain full node in the background, and return the child process handle
        let chain_process = bootstrap
            .start_chain_full_nodes(&chain_home_dir, &chain_config, &genesis_config)
            .await?;

        // Build the chain context from the bootstrap parameters
        let chain = bootstrap
            .build_chain_driver(genesis_config, chain_config, wallets, chain_process)
            .await?;

        Ok(chain)
    }
}
