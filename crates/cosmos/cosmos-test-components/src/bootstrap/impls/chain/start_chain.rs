use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::os::child_process::CanStartChildProcess;
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::bootstrap::traits::chain::start_chain::ChainFullNodeStarter;
use crate::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use crate::bootstrap::traits::types::chain_node_config::HasChainNodeConfigType;
use crate::bootstrap::types::chain_node_config::CosmosChainNodeConfig;

pub struct StartCosmosChain;

#[async_trait]
impl<Bootstrap, Runtime> ChainFullNodeStarter<Bootstrap> for StartCosmosChain
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasErrorType
        + HasChainCommandPath
        + HasChainNodeConfigType<ChainNodeConfig = CosmosChainNodeConfig>
        + CanRaiseError<Runtime::Error>,
    Runtime: HasFilePathType + CanStartChildProcess,
{
    async fn start_chain_full_node(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        chain_config: &CosmosChainNodeConfig,
    ) -> Result<Runtime::ChildProcess, Bootstrap::Error> {
        let chain_command = bootstrap.chain_command_path();

        let args = [
            "--home",
            &Runtime::file_path_to_string(chain_home_dir),
            "start",
            "--pruning",
            "nothing",
            "--grpc.address",
            &format!("localhost:{}", chain_config.grpc_port),
            "--rpc.laddr",
            &format!("tcp://localhost:{}", chain_config.rpc_port),
        ];

        let stdout_path = Runtime::join_file_path(
            chain_home_dir,
            &Runtime::file_path_from_string("stdout.log"),
        );

        let stderr_path = Runtime::join_file_path(
            chain_home_dir,
            &Runtime::file_path_from_string("stderr.log"),
        );

        let child_process = bootstrap
            .runtime()
            .start_child_process(
                chain_command,
                &args,
                &[],
                Some(&stdout_path),
                Some(&stderr_path),
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(child_process)
    }
}
