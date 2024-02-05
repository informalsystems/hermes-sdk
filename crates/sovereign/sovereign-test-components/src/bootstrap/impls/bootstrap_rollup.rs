use cgp_core::CanRaiseError;
use hermes_celestia_test_components::bootstrap::traits::types::bridge_driver::HasBridgeDriverType;
use hermes_cosmos_test_components::chain_driver::types::wallet::CosmosTestWallet;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::fields::wallet::HasWallets;
use hermes_test_components::chain_driver::traits::types::wallet::HasWalletType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::create_dir::CanCreateDir;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;

use crate::bootstrap::traits::bootstrap_rollup::RollupBootstrapper;
use crate::bootstrap::traits::build_rollup_driver::CanBuildRollupDriver;
use crate::bootstrap::traits::generate_rollup_genesis::CanGenerateRollupGenesis;
use crate::bootstrap::traits::generate_rollup_wallets::CanGenerateRollupWallets;
use crate::bootstrap::traits::init_rollup_config::CanInitRollupConfig;
use crate::bootstrap::traits::rollup_store_dir::HasRollupStoreDir;
use crate::bootstrap::traits::types::rollup_driver::HasRollupDriverType;
use crate::bootstrap::traits::write_rollup_genesis::CanWriteRollupGenesis;

pub struct BootstrapSovereignRollup;

impl<Bootstrap, ChainDriver, RollupDriver, Runtime> RollupBootstrapper<Bootstrap>
    for BootstrapSovereignRollup
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + HasBridgeDriverType
        + HasRollupDriverType<RollupDriver = RollupDriver>
        + HasRollupStoreDir
        + CanInitRollupConfig
        + CanGenerateRollupWallets
        + CanGenerateRollupGenesis
        + CanWriteRollupGenesis
        + CanBuildRollupDriver
        + CanRaiseError<&'static str>
        + CanRaiseError<Runtime::Error>,
    ChainDriver: HasWallets<Wallet = CosmosTestWallet>,
    RollupDriver: HasWalletType,
    Runtime: HasFilePathType + CanCreateDir,
{
    async fn bootstrap_rollup(
        bootstrap: &Bootstrap,
        chain_driver: &ChainDriver,
        bridge_driver: &Bootstrap::BridgeDriver,
        rollup_id: &str,
    ) -> Result<RollupDriver, Bootstrap::Error> {
        let rollup_home_dir = Runtime::join_file_path(
            bootstrap.rollup_store_dir(),
            &Runtime::file_path_from_string(rollup_id),
        );

        bootstrap
            .runtime()
            .create_dir(&rollup_home_dir)
            .await
            .map_err(Bootstrap::raise_error)?;

        let rollup_config = bootstrap
            .init_rollup_config(&rollup_home_dir, bridge_driver)
            .await?;

        // TODO: Use `HasWalletAt<SequencerWallet, 0>` instead once we define a
        // `CelestiaChainDriver` context that implements that.
        let sequencer_wallet = chain_driver
            .wallets()
            .iter()
            .find(|wallet| wallet.id.starts_with("sequencer"))
            .ok_or_else(|| {
                Bootstrap::raise_error("expected chain driver to contain sequencer wallet")
            })?;

        let rollup_wallets = bootstrap.generate_rollup_wallets().await?;

        let rollup_genesis = bootstrap
            .generate_rollup_genesis(
                ChainDriver::wallet_address(sequencer_wallet),
                &rollup_wallets,
            )
            .await?;

        bootstrap
            .write_rollup_genesis(&rollup_home_dir, &rollup_genesis)
            .await?;

        let rollup_driver = bootstrap
            .build_rollup_driver(rollup_config, rollup_genesis)
            .await?;

        // TODO: spawn rollup child process

        Ok(rollup_driver)
    }
}
