use core::time::Duration;

use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_relayer_components::runtime::traits::sleep::CanSleep;
use hermes_test_components::chain_driver::traits::types::chain::{HasChain, HasChainType};
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::child_process::CanStartChildProcess;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;

use crate::bootstrap::traits::start_bridge::BridgeStarter;
use crate::bootstrap::traits::types::bridge_config::HasBridgeConfigType;
use crate::types::bridge_config::CelestiaBridgeConfig;

pub struct StartCelestiaBridge;

impl<Bootstrap, ChainDriver, Chain, Runtime> BridgeStarter<Bootstrap> for StartCelestiaBridge
where
    Bootstrap: HasChainType<Chain = Chain>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + HasBridgeConfigType<BridgeConfig = CelestiaBridgeConfig>
        + HasRuntime<Runtime = Runtime>
        + CanRaiseError<Runtime::Error>,
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

        // println!("running command: HOME={} celestia {}",
        //     Runtime::file_path_to_string(bridge_home_dir),
        //     args.join(" ")
        // );

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
