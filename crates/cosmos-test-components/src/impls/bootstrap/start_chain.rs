use cgp_core::prelude::*;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;

use crate::traits::bootstrap::start_chain::ChainFullNodeStarter;
use crate::traits::fields::chain_command_path::HasChainCommandPath;
use ibc_test_components::runtime::traits::child_process::CanStartChildProcess;
use ibc_test_components::runtime::traits::types::file_path::HasFilePathType;

pub struct StartCosmosChain;

#[async_trait]
impl<Bootstrap, Runtime> ChainFullNodeStarter<Bootstrap> for StartCosmosChain
where
    Bootstrap: HasRuntime<Runtime = Runtime> + HasErrorType + HasChainCommandPath,
    Runtime: HasFilePathType + CanStartChildProcess,
{
    async fn start_chain_full_node(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
    ) -> Result<Runtime::ChildProcess, Bootstrap::Error> {
        let chain_command = bootstrap.chain_command_path();

        let args = [
            "--home",
            &Runtime::file_path_to_string(chain_home_dir),
            "--start",
            "--pruning",
            "nothing",
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
            .start_child_process(chain_command, &args, Some(&stdout_path), Some(&stderr_path))
            .await
            .map_err(Bootstrap::runtime_error)?;

        Ok(child_process)
    }
}
