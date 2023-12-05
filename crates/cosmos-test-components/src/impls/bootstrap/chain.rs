use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::traits::bootstrap::chain::ChainBootstrapper;
use ibc_test_components::traits::chain::types::chain::HasChainType;

use crate::traits::bootstrap::start_chain::CanStartChainFullNode;
use crate::traits::generator::generate_chain_id::CanGenerateChainId;
use crate::traits::generator::generate_wallet_config::CanGenerateWalletConfigs;
use crate::traits::genesis::add_genesis_wallet::CanAddWalletToGenesis;
use crate::traits::genesis::collect_gentxs::CanCollectGenesisTransactions;
use crate::traits::initializers::init_chain_config::CanInitChainConfig;
use crate::traits::initializers::init_chain_data::CanInitChainData;
use crate::traits::initializers::init_chain_home_dir::CanInitChainHomeDir;
use crate::traits::initializers::init_genesis_config::CanInitGenesisConfig;
use ibc_test_components::runtime::traits::types::child_process::HasChildProcessType;
use ibc_test_components::runtime::traits::types::file_path::HasFilePathType;

pub struct BoostrapCosmosChain;

#[async_trait]
impl<Bootstrap, Runtime, Chain> ChainBootstrapper<Bootstrap> for BoostrapCosmosChain
where
    Bootstrap: HasErrorType
        + HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + CanGenerateChainId
        + CanInitChainHomeDir
        + CanInitChainData
        + CanInitGenesisConfig
        + CanGenerateWalletConfigs
        + CanAddWalletToGenesis
        + CanCollectGenesisTransactions
        + CanInitChainConfig
        + CanStartChainFullNode,
    Runtime: HasFilePathType + HasChildProcessType,
    Chain: HasChainIdType,
{
    async fn bootstrap_chain(
        bootstrap: &Bootstrap,
        chain_id_prefix: &str,
    ) -> Result<Bootstrap::Chain, Bootstrap::Error> {
        let chain_id = bootstrap.generate_chain_id(chain_id_prefix).await;

        let chain_home_dir = bootstrap.init_chain_home_dir(&chain_id).await?;

        // Run the `init` chain CLI subcommand to initialize the chain data files on the
        // given chain home directory.
        bootstrap
            .init_chain_data(&chain_home_dir, &chain_id)
            .await?;

        // Initialize (or update) the genesis config file on the chain home directory
        bootstrap.init_genesis_config(&chain_home_dir).await?;

        let _wallets = {
            // Generate and add wallets to the genesis config
            let wallet_configs = bootstrap.generate_wallet_configs().await?;

            let mut wallets = Vec::new();

            for wallet_config in wallet_configs {
                let wallet = bootstrap
                    .add_wallet_to_genesis(&chain_home_dir, &chain_id, &wallet_config)
                    .await?;
                wallets.push(wallet);
            }

            wallets
        };

        bootstrap
            .collect_genesis_transactions(&chain_home_dir)
            .await?;

        let _chain_config = bootstrap.init_chain_config(&chain_home_dir).await?;

        let _child_process = bootstrap.start_chain_full_node(&chain_home_dir).await?;

        todo!()
    }
}
