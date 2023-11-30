use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;

use crate::traits::commands::init_chain::InitChainCommandRunner;
use crate::traits::fields::chain_command_path::HasChainCommandPath;
use crate::traits::io::exec_command::CanExecCommand;
use crate::traits::types::file_path::HasFilePathType;

pub struct InitializeCosmosChain;

#[async_trait]
impl<Bootstrap> InitChainCommandRunner<Bootstrap> for InitializeCosmosChain
where
    Bootstrap: HasChainIdType + HasFilePathType + HasChainCommandPath + CanExecCommand,
{
    async fn run_init_chain_command(
        bootstrap: &Bootstrap,
        chain_id: &Bootstrap::ChainId,
        chain_home_dir: &Bootstrap::FilePath,
    ) -> Result<(), Bootstrap::Error> {
        let chain_id = chain_id.to_string();
        let chain_command_path = bootstrap.chain_command_path();

        bootstrap
            .exec_command(
                "initialize cosmos chain",
                chain_command_path,
                &[
                    "--home",
                    &Bootstrap::file_path_to_string(chain_home_dir),
                    "--chain-id",
                    &chain_id,
                    "init",
                    &chain_id,
                ],
            )
            .await?;

        Ok(())
    }
}
