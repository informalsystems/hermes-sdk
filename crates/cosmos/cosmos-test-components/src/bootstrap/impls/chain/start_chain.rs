use cgp::prelude::*;
use hermes_core::runtime_components::traits::{CanStartChildProcess, HasFilePathType, HasRuntime};

use crate::bootstrap::traits::{
    ChainFullNodeStarter, ChainFullNodeStarterComponent, HasChainCommandPath,
    HasChainGenesisConfigType, HasChainNodeConfigType,
};
use crate::bootstrap::types::{CosmosChainNodeConfig, CosmosGenesisConfig};

#[cgp_new_provider(ChainFullNodeStarterComponent)]
impl<Bootstrap, Runtime> ChainFullNodeStarter<Bootstrap> for StartCosmosChain
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasAsyncErrorType
        + HasChainCommandPath
        + HasChainNodeConfigType<ChainNodeConfig = CosmosChainNodeConfig>
        + HasChainGenesisConfigType<ChainGenesisConfig = CosmosGenesisConfig>
        + CanRaiseAsyncError<Runtime::Error>,
    Runtime: HasFilePathType + CanStartChildProcess,
{
    async fn start_chain_full_nodes(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        chain_config: &CosmosChainNodeConfig,
        _chain_genesis_config: &CosmosGenesisConfig,
    ) -> Result<Vec<Runtime::ChildProcess>, Bootstrap::Error> {
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
            // "--minimum-gas-prices",
            // &format!("0.01{}", chain_genesis_config.staking_denom),
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

        Ok(vec![child_process])
    }
}
