use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::chain::{HasChain, HasChainType};
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::child_process::CanStartChildProcess;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;

use crate::bootstrap::traits::bootstrap_bridge::BridgeBootstrapper;
use crate::bootstrap::traits::bridge_store_dir::HasBridgeStoreDir;
use crate::bootstrap::traits::import_bridge_key::CanImportBridgeKey;
use crate::bootstrap::traits::init_bridge_config::CanInitBridgeConfig;
use crate::bootstrap::traits::init_bridge_data::CanInitBridgeData;

pub struct BootstrapCelestiaBridge;

impl<Bootstrap, Chain, ChainDriver, Runtime> BridgeBootstrapper<Bootstrap>
    for BootstrapCelestiaBridge
where
    Bootstrap: HasChainType<Chain = Chain>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + HasRuntime<Runtime = Runtime>
        + HasBridgeStoreDir
        + CanInitBridgeData
        + CanImportBridgeKey
        + CanInitBridgeConfig
        + CanRaiseError<Runtime::Error>,
    ChainDriver: HasChain<Chain = Chain> + HasRuntime<Runtime = Runtime>,
    Chain: HasChainId<ChainId = ChainId>,
    Runtime: HasFilePathType + CanStartChildProcess,
{
    async fn bootstrap_bridge(
        bootstrap: &Bootstrap,
        chain_driver: &ChainDriver,
    ) -> Result<Runtime::ChildProcess, Bootstrap::Error> {
        let runtime = bootstrap.runtime();
        let chain = chain_driver.chain();

        let chain_id = chain.chain_id();
        let chain_id_str = chain_id.to_string();
        let bridge_store_dir = bootstrap.bridge_store_dir();

        let bridge_home_dir = Runtime::join_file_path(
            bridge_store_dir,
            &Runtime::file_path_from_string(&chain_id_str),
        );

        bootstrap
            .init_bridge_data(&bridge_home_dir, chain_id)
            .await?;

        bootstrap
            .init_bridge_config(&bridge_home_dir, chain_driver)
            .await?;

        bootstrap
            .import_bridge_key(&bridge_home_dir, chain_driver)
            .await?;

        let stdout_path = Runtime::join_file_path(
            &bridge_home_dir,
            &Runtime::file_path_from_string("stdout.log"),
        );

        let stderr_path = Runtime::join_file_path(
            &bridge_home_dir,
            &Runtime::file_path_from_string("stderr.log"),
        );

        let child = runtime
            .start_child_process(
                &Runtime::file_path_from_string("celestia"),
                &[
                    "bridge",
                    "start",
                    "--keyring.accname",
                    "bridge",
                    "--p2p.network",
                    &chain_id_str,
                ],
                &[("HOME", &Runtime::file_path_to_string(&bridge_home_dir))],
                Some(&stdout_path),
                Some(&stderr_path),
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(child)
    }
}
