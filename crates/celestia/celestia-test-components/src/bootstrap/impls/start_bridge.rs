use core::time::Duration;

use hermes_prelude::*;
use hermes_relayer_components::chain::traits::HasChainId;
use hermes_runtime_components::traits::{
    CanSleep, CanStartChildProcess, HasFilePathType, HasRuntime,
};
use hermes_test_components::chain_driver::traits::{HasChain, HasChainType};
use hermes_test_components::driver::traits::HasChainDriverType;
use ibc::core::host::types::identifiers::ChainId;

use crate::bootstrap::traits::start_bridge::{BridgeStarter, BridgeStarterComponent};
use crate::bootstrap::traits::types::bridge_config::HasBridgeConfigType;
use crate::types::bridge_config::CelestiaBridgeConfig;

pub struct StartCelestiaBridge;

#[cgp_provider(BridgeStarterComponent)]
impl<Bootstrap, ChainDriver, Chain, Runtime> BridgeStarter<Bootstrap> for StartCelestiaBridge
where
    Bootstrap: HasChainType<Chain = Chain>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + HasBridgeConfigType<BridgeConfig = CelestiaBridgeConfig>
        + HasRuntime<Runtime = Runtime>
        + CanRaiseAsyncError<Runtime::Error>,
    ChainDriver: HasChain<Chain = Chain> + HasRuntime<Runtime = Runtime>,
    Chain: HasChainId<ChainId = ChainId>,
    Runtime: HasFilePathType + CanStartChildProcess + CanSleep,
{
    async fn start_bridge(
        bootstrap: &Bootstrap,
        bridge_home_dir: &Runtime::FilePath,
        bridge_config: &CelestiaBridgeConfig,
        chain_driver: &ChainDriver,
    ) -> Result<Runtime::ChildProcess, Bootstrap::Error> {
        let stdout_path = Runtime::join_file_path(
            bridge_home_dir,
            &Runtime::file_path_from_string("stdout.log"),
        );

        let stderr_path = Runtime::join_file_path(
            bridge_home_dir,
            &Runtime::file_path_from_string("stderr.log"),
        );

        let args = [
            "bridge",
            "start",
            "--core.ip",
            "127.0.0.1",
            "--core.grpc.port",
            &bridge_config.node_grpc_port.to_string(),
            "--core.rpc.port",
            &bridge_config.node_rpc_port.to_string(),
            "--keyring.accname",
            "sequencer",
            "--keyring.backend",
            "test",
            "--p2p.network",
            &chain_driver.chain().chain_id().to_string(),
        ];

        let child = bootstrap
            .runtime()
            .start_child_process(
                &Runtime::file_path_from_string("celestia"),
                &args,
                &[("HOME", &Runtime::file_path_to_string(bridge_home_dir))],
                Some(&stdout_path),
                Some(&stderr_path),
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        // Wait for the bridge node to start
        bootstrap.runtime().sleep(Duration::from_secs(1)).await;

        Ok(child)
    }
}
