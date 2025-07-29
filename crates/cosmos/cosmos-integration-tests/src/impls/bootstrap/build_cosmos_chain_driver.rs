use alloc::collections::BTreeMap;
use std::path::PathBuf;

use hermes_core::runtime_components::traits::{
    HasChildProcessType, HasFilePathType, HasRuntimeType,
};
use hermes_core::test_components::chain_driver::traits::HasChainType;
use hermes_core::test_components::driver::traits::HasChainDriverType;
use hermes_cosmos_core::test_components::bootstrap::traits::{
    ChainDriverBuilder, ChainDriverBuilderComponent, DenomForStaking, DenomForTransfer,
    HasChainCommandPath, HasChainGenesisConfigType, HasChainNodeConfigType, HasGenesisDenom,
};
use hermes_cosmos_core::test_components::bootstrap::types::{
    CosmosChainNodeConfig, CosmosGenesisConfig,
};
use hermes_cosmos_core::test_components::chain::types::CosmosTestWallet;
use hermes_cosmos_relayer::contexts::CosmosChain;
use hermes_prelude::*;
use tokio::process::Child;

use crate::contexts::CosmosChainDriver;
use crate::traits::CanBuildChainWithNodeConfig;

#[cgp_new_provider(ChainDriverBuilderComponent)]
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
        + CanRaiseAsyncError<&'static str>,
    Runtime: HasFilePathType<FilePath = PathBuf> + HasChildProcessType<ChildProcess = Child>,
{
    async fn build_chain_driver(
        bootstrap: &Bootstrap,
        genesis_config: CosmosGenesisConfig,
        chain_node_config: CosmosChainNodeConfig,
        wallets: BTreeMap<String, CosmosTestWallet>,
        chain_processes: Vec<Child>,
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

        let additional_relayer_wallet = wallets
            .get("relayer-2")
            .ok_or_else(|| {
                Bootstrap::raise_error(
                    "expect relayer-2 wallet to be provided in the list of test wallets",
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
            .build_chain_with_node_config(
                &chain_node_config,
                &genesis_config,
                &relayer_wallet,
                vec![&additional_relayer_wallet],
            )
            .await?;

        let chain_command_path = bootstrap.chain_command_path().clone();

        let chain_driver = CosmosChainDriver {
            chain,
            chain_command_path,
            chain_node_config,
            genesis_config,
            chain_processes,
            validator_wallet,
            relayer_wallet,
            user_wallet_a,
            user_wallet_b,
            wallets,
        };

        Ok(chain_driver)
    }
}
