use core::time::Duration;

use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_cosmos_test_components::bootstrap::types::chain_config::CosmosChainConfig;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::types::aliases::ChainId;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_relayer_components::runtime::traits::sleep::CanSleep;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::runtime::traits::child_process::CanStartChildProcess;
use hermes_test_components::runtime::traits::copy_file::CanCopyFile;
use hermes_test_components::runtime::traits::create_dir::CanCreateDir;
use hermes_test_components::runtime::traits::exec_command::CanExecCommandWithEnvs;
use hermes_test_components::runtime::traits::types::child_process::{
    ChildProcess, HasChildProcessType,
};
use hermes_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

use crate::bootstrap::traits::bridge_store_dir::HasBridgeStoreDir;

#[async_trait]
pub trait CanInitCelestiaBridge: HasChainType + HasRuntime + HasErrorType
where
    Self::Chain: HasChainIdType,
    Self::Runtime: HasFilePathType + HasChildProcessType,
{
    async fn init_celestia_bridge(
        &self,
        chain_home_dir: &FilePath<Self::Runtime>,
        chain_id: &ChainId<Self::Chain>,
        chain_config: &CosmosChainConfig,
    ) -> Result<ChildProcess<Self::Runtime>, Self::Error>;
}

impl<Bootstrap, Chain, Runtime> CanInitCelestiaBridge for Bootstrap
where
    Bootstrap: HasChainType<Chain = Chain>
        + HasRuntime<Runtime = Runtime>
        + HasBridgeStoreDir
        + CanRaiseError<Runtime::Error>,
    Chain: HasChainIdType,
    Runtime: HasFilePathType
        + HasChildProcessType
        + CanExecCommandWithEnvs
        + CanCreateDir
        + CanCopyFile
        + CanStartChildProcess
        + CanSleep,
{
    async fn init_celestia_bridge(
        &self,
        chain_home_dir: &Runtime::FilePath,
        chain_id: &Chain::ChainId,
        chain_config: &CosmosChainConfig,
    ) -> Result<Runtime::ChildProcess, Self::Error> {
        let runtime = self.runtime();
        let chain_id_str = chain_id.to_string();
        let bridge_store_dir = self.bridge_store_dir();

        let bridge_home_dir = Runtime::join_file_path(
            bridge_store_dir,
            &Runtime::file_path_from_string(&chain_id_str),
        );

        runtime
            .create_dir(&bridge_home_dir)
            .await
            .map_err(Bootstrap::raise_error)?;

        runtime
            .exec_command_with_envs(
                &Runtime::file_path_from_string("celestia"),
                &["bridge", "init", "--p2p.network", &chain_id_str],
                &[("HOME", &Runtime::file_path_to_string(&bridge_home_dir))],
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        let bridge_key_source_path = Runtime::join_file_path(
            chain_home_dir,
            &Runtime::file_path_from_string("keyring-test/bridge.info"),
        );

        let bridge_key_destination_path = Runtime::join_file_path(
            &bridge_home_dir,
            &Runtime::file_path_from_string(&format!(
                ".celestia-bridge-{chain_id_str}/keys/keyring-test/bridge.info"
            )),
        );

        runtime
            .copy_file(&bridge_key_source_path, &bridge_key_destination_path)
            .await
            .map_err(Bootstrap::raise_error)?;

        let stdout_path = Runtime::join_file_path(
            &bridge_home_dir,
            &Runtime::file_path_from_string("stdout.log"),
        );

        let stderr_path = Runtime::join_file_path(
            &bridge_home_dir,
            &Runtime::file_path_from_string("stderr.log"),
        );

        let args = [
            "bridge",
            "start",
            "--keyring.accname",
            "bridge",
            "--core.ip",
            "127.0.0.1",
            "--core.rpc.port",
            &chain_config.rpc_port.to_string(),
            "--core.grpc.port",
            &chain_config.grpc_port.to_string(),
            "--p2p.network",
            &chain_id_str,
        ];

        println!("running celestia bridge with args: {:?}", args);

        runtime.sleep(Duration::from_secs(999999)).await;

        let child = runtime
            .start_child_process(
                &Runtime::file_path_from_string("celestia"),
                &args,
                &[("HOME", &Runtime::file_path_to_string(&bridge_home_dir))],
                Some(&stdout_path),
                Some(&stderr_path),
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(child)
    }
}
