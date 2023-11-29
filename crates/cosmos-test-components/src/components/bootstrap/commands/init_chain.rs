use std::path::Path;

use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;

use crate::traits::bootstrap::commands::init_chain::InitChainCommandRunner;
use crate::traits::chain_command_path::HasChainCommandPath;
use crate::traits::exec_command::CanExecCommand;

pub struct InitializeCosmosChain;

#[async_trait]
impl<Bootstrap> InitChainCommandRunner<Bootstrap> for InitializeCosmosChain
where
    Bootstrap: HasChainIdType + HasChainCommandPath + CanExecCommand,
{
    async fn run_init_chain_command(
        bootstrap: &Bootstrap,
        chain_id: &Bootstrap::ChainId,
        chain_home_dir: &Path,
    ) -> Result<(), Bootstrap::Error> {
        let chain_id = chain_id.to_string();
        let chain_command_path = bootstrap.chain_command_path();

        bootstrap
            .exec_command(
                "initialize cosmos chain",
                chain_command_path,
                &[
                    "--home",
                    &chain_home_dir.to_string_lossy(),
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
