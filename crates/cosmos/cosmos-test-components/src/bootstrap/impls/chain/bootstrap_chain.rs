use alloc::collections::BTreeMap;

use cgp::prelude::*;
use hermes_relayer_components::chain::traits::HasChainIdType;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::os::child_process::HasChildProcessType;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::bootstrap::traits::chain::{
    ChainBootstrapper, ChainBootstrapperComponent,
};
use hermes_test_components::chain::traits::types::wallet::HasWalletType;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::bootstrap::traits::chain::build_chain_driver::CanBuildChainDriver;
use crate::bootstrap::traits::chain::start_chain::CanStartChainFullNodes;
use crate::bootstrap::traits::generator::generate_chain_id::CanGenerateChainId;
use crate::bootstrap::traits::generator::generate_wallet_config::CanGenerateWalletConfigs;
use crate::bootstrap::traits::genesis::add_genesis_wallet::CanAddWalletToGenesis;
use crate::bootstrap::traits::genesis::collect_gentxs::CanCollectGenesisTransactions;
use crate::bootstrap::traits::initializers::init_chain_config::CanInitChainNodeConfig;
use crate::bootstrap::traits::initializers::init_chain_data::CanInitChainData;
use crate::bootstrap::traits::initializers::init_chain_home_dir::CanInitChainHomeDir;
use crate::bootstrap::traits::initializers::init_genesis_config::CanInitChainGenesisConfig;

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
