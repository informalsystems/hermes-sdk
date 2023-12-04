use cgp_core::prelude::*;

use crate::traits::bootstrap::start_chain::ChainFullNodeStarter;
use crate::traits::fields::chain_command_path::HasChainCommandPath;
use crate::traits::io::child_process::CanStartChildProcess;
use crate::traits::types::io::file_path::HasFilePathType;

pub struct StartCosmosChain;

#[async_trait]
impl<Bootstrap> ChainFullNodeStarter<Bootstrap> for StartCosmosChain
where
    Bootstrap: CanStartChildProcess + HasFilePathType + HasErrorType + HasChainCommandPath,
{
    async fn start_chain_full_node(
        bootstrap: &Bootstrap,
        chain_home_dir: &Bootstrap::FilePath,
    ) -> Result<Bootstrap::ChildProcess, Bootstrap::Error> {
        let chain_command = bootstrap.chain_command_path();

        let args = [
            "--home",
            &Bootstrap::file_path_to_string(chain_home_dir),
            "--start",
            "--pruning",
            "nothing",
        ];

        let stdout_path = Bootstrap::join_file_path(
            chain_home_dir,
            &Bootstrap::file_path_from_string("stdout.log"),
        );

        let stderr_path = Bootstrap::join_file_path(
            chain_home_dir,
            &Bootstrap::file_path_from_string("stderr.log"),
        );

        let child_process = bootstrap
            .start_child_process(chain_command, &args, Some(&stdout_path), Some(&stderr_path))
            .await?;

        Ok(child_process)
    }
}
