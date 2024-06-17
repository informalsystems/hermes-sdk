use cgp_core::CanRaiseError;
use hermes_celestia_test_components::bootstrap::traits::types::bridge_driver::HasBridgeDriverType;
use hermes_cosmos_test_components::chain::types::wallet::CosmosTestWallet;
use hermes_runtime_components::traits::fs::create_dir::CanCreateDir;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::os::child_process::HasChildProcessType;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_runtime_components::traits::sleep::CanSleep;
use hermes_sovereign_chain_components::sovereign::traits::chain::rollup::HasRollupType;
use hermes_test_components::chain::traits::types::wallet::HasWalletType;
use hermes_test_components::chain_driver::traits::fields::wallet::HasWallets;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::bootstrap::traits::bootstrap_rollup::RollupBootstrapper;
use crate::bootstrap::traits::build_rollup_driver::CanBuildRollupDriver;
use crate::bootstrap::traits::generate_rollup_genesis::CanGenerateRollupGenesis;
use crate::bootstrap::traits::generate_rollup_wallets::CanGenerateRollupWallets;
use crate::bootstrap::traits::init_rollup_node_config::CanInitRollupNodeConfig;
use crate::bootstrap::traits::rollup_store_dir::HasRollupStoreDir;
use crate::bootstrap::traits::start_rollup::CanStartRollup;
use crate::bootstrap::traits::types::rollup_driver::HasRollupDriverType;
use crate::bootstrap::traits::write_rollup_genesis::CanWriteRollupGenesis;

pub struct BootstrapSovereignRollup;

impl<Bootstrap, Chain, ChainDriver, Rollup, Runtime> RollupBootstrapper<Bootstrap>
    for BootstrapSovereignRollup
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + HasRollupType<Rollup = Rollup>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + HasBridgeDriverType
        + HasRollupDriverType
        + HasRollupStoreDir
        + CanInitRollupNodeConfig
        + CanGenerateRollupWallets
        + CanGenerateRollupGenesis
        + CanWriteRollupGenesis
        + CanStartRollup
        + CanBuildRollupDriver
        + CanRaiseError<&'static str>
        + CanRaiseError<Runtime::Error>,
    Chain: HasWalletType<Wallet = CosmosTestWallet>,
    ChainDriver: HasChainType<Chain = Chain> + HasWallets,
    Rollup: HasWalletType,
    Runtime: HasFilePathType + HasChildProcessType + CanCreateDir + CanSleep,
{
    async fn bootstrap_rollup(
        bootstrap: &Bootstrap,
        chain_driver: &ChainDriver,
        bridge_driver: &Bootstrap::BridgeDriver,
        rollup_id: &str,
    ) -> Result<Bootstrap::RollupDriver, Bootstrap::Error> {
        let rollup_home_dir = Runtime::join_file_path(
            bootstrap.rollup_store_dir(),
            &Runtime::file_path_from_string(rollup_id),
        );

        bootstrap
            .runtime()
            .create_dir(&rollup_home_dir)
            .await
            .map_err(Bootstrap::raise_error)?;

        // TODO: Use `HasWalletAt<SequencerWallet, 0>` instead once we define a
        // `CelestiaChainDriver` context that implements that.
        let sequencer_wallet = chain_driver.wallets().get("sequencer").ok_or_else(|| {
            Bootstrap::raise_error("expected chain driver to contain sequencer wallet")
        })?;

        let sequencer_address = Chain::wallet_address(sequencer_wallet);

        let rollup_node_config = bootstrap
            .init_rollup_node_config(&rollup_home_dir, bridge_driver, sequencer_address)
            .await?;

        let rollup_wallets = bootstrap.generate_rollup_wallets().await?;

        let rollup_genesis = bootstrap
            .generate_rollup_genesis(sequencer_address, &rollup_wallets)
            .await?;

        bootstrap
            .write_rollup_genesis(&rollup_home_dir, &rollup_genesis)
            .await?;

        let rollup_process = bootstrap.start_rollup(&rollup_home_dir).await?;

        bootstrap
            .runtime()
            .sleep(core::time::Duration::from_secs(2))
            .await;

        let rollup_driver = bootstrap
            .build_rollup_driver(
                rollup_node_config,
                rollup_genesis,
                rollup_wallets,
                rollup_process,
            )
            .await?;

        // TODO: spawn rollup child process

        Ok(rollup_driver)
    }
}
