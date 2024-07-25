use alloc::collections::BTreeMap;
use alloc::sync::Arc;
use std::path::PathBuf;
use tokio::sync::Mutex;

use cgp_core::error::CanRaiseError;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_cosmos_test_components::bootstrap::traits::chain::build_chain_driver::ChainDriverBuilder;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use hermes_cosmos_test_components::bootstrap::traits::fields::denom::{
    DenomForStaking, DenomForTransfer, HasGenesisDenom,
};
use hermes_cosmos_test_components::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use hermes_cosmos_test_components::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use hermes_cosmos_test_components::bootstrap::types::chain_node_config::CosmosChainNodeConfig;
use hermes_cosmos_test_components::bootstrap::types::genesis_config::CosmosGenesisConfig;
use hermes_cosmos_test_components::chain::types::wallet::CosmosTestWallet;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::os::child_process::HasChildProcessType;
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use tokio::process::Child;

use crate::contexts::chain_driver::CosmosChainDriver;
use crate::traits::bootstrap::build_chain::CanBuildChainWithNodeConfig;

pub struct BuildCosmosChainDriver;

impl<Bootstrap, Runtime> ChainDriverBuilder<Bootstrap> for BuildCosmosChainDriver
where
    Bootstrap: HasChainDriverType<ChainDriver = CosmosChainDriver>
        + HasChainType<Chain = CosmosChain>
        + HasChainNodeConfigType<ChainNodeConfig = CosmosChainNodeConfig>
        + HasChainGenesisConfigType<ChainGenesisConfig = CosmosGenesisConfig>
        + HasRuntimeType<Runtime = Runtime>
        + CanBuildChainWithNodeConfig
        + HasGenesisDenom<DenomForStaking>
        + HasGenesisDenom<DenomForTransfer>
        + HasChainCommandPath
        + CanRaiseError<&'static str>,
    Runtime: HasFilePathType<FilePath = PathBuf> + HasChildProcessType<ChildProcess = Child>,
{
    async fn build_chain_driver(
        bootstrap: &Bootstrap,
        genesis_config: CosmosGenesisConfig,
        chain_node_config: CosmosChainNodeConfig,
        wallets: BTreeMap<String, CosmosTestWallet>,
        chain_process: Child,
    ) -> Result<CosmosChainDriver, Bootstrap::Error> {
        let validator_wallet = wallets
            .get("validator")
            .ok_or_else(|| {
                Bootstrap::raise_error(
                    "expect validator wallet to be provided in the list of test wallets",
                )
            })?
            .clone();

        let relayer_wallet = wallets
            .get("relayer")
            .ok_or_else(|| {
                Bootstrap::raise_error(
                    "expect relayer wallet to be provided in the list of test wallets",
                )
            })?
            .clone();

        let user_wallet_a = wallets
            .get("user1")
            .ok_or_else(|| {
                Bootstrap::raise_error(
                    "expect user1 wallet to be provided in the list of test wallets",
                )
            })?
            .clone();

        let user_wallet_b = wallets
            .get("user2")
            .ok_or_else(|| {
                Bootstrap::raise_error(
                    "expect user2 wallet to be provided in the list of test wallets",
                )
            })?
            .clone();

        let chain = bootstrap
            .build_chain_with_node_config(&chain_node_config, &relayer_wallet)
            .await?;

        let chain_command_path = bootstrap.chain_command_path().clone();

        let chain_driver = CosmosChainDriver {
            chain,
            chain_command_path,
            chain_node_config,
            genesis_config,
            chain_process: Arc::new(Mutex::new(Some(chain_process))),
            validator_wallet,
            relayer_wallet,
            user_wallet_a,
            user_wallet_b,
            wallets,
        };

        Ok(chain_driver)
    }
}
