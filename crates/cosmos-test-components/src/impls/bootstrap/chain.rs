use cgp_core::prelude::*;
use ibc_test_components::traits::bootstrap::chain::ChainBootstrapper;

use crate::traits::generator::generate_chain_id::CanGenerateChainId;
use crate::traits::generator::generate_wallet_config::CanGenerateWalletConfigs;
use crate::traits::genesis::add_genesis_wallet::CanAddWalletToGenesis;
use crate::traits::genesis::collect_gentxs::CanCollectGenesisTransactions;
use crate::traits::initializers::init_chain_config::CanInitChainConfig;
use crate::traits::initializers::init_chain_data::CanInitChainData;
use crate::traits::initializers::init_chain_home_dir::CanInitChainHomeDir;
use crate::traits::initializers::init_genesis_config::CanInitGenesisConfig;
use crate::traits::io::reserve_port::CanReserveTcpPort;

pub struct BoostrapCosmosChain;

#[async_trait]
impl<Bootstrap, Chain> ChainBootstrapper<Bootstrap, Chain> for BoostrapCosmosChain
where
    Bootstrap: HasErrorType
        + CanGenerateChainId
        + CanInitChainHomeDir
        + CanReserveTcpPort
        + CanInitChainData
        + CanInitGenesisConfig
        + CanGenerateWalletConfigs
        + CanAddWalletToGenesis
        + CanCollectGenesisTransactions
        + CanInitChainConfig,
{
    async fn bootstrap_chain(
        bootstrap: &Bootstrap,
        chain_id_prefix: &str,
    ) -> Result<Chain, Bootstrap::Error> {
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

        todo!()
    }
}
