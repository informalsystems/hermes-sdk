use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;

use crate::traits::fields::chain_command_path::HasChainCommandPath;
use crate::traits::initializers::init_chain_data::ChainDataInitializer;
use crate::traits::runtime::exec_command::CanExecCommand;
use crate::traits::runtime::types::file_path::HasFilePathType;

pub struct InitCosmosChainData;

#[async_trait]
impl<Bootstrap, Runtime> ChainDataInitializer<Bootstrap> for InitCosmosChainData
where
    Bootstrap: HasChainIdType + HasRuntime<Runtime = Runtime> + HasChainCommandPath,
    Runtime: HasFilePathType + CanExecCommand,
{
    async fn init_chain_data(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        chain_id: &Bootstrap::ChainId,
    ) -> Result<(), Bootstrap::Error> {
        let chain_id = chain_id.to_string();
        let chain_command_path = bootstrap.chain_command_path();

        bootstrap
            .runtime()
            .exec_command(
                "initialize cosmos chain",
                chain_command_path,
                &[
                    "--home",
                    &Runtime::file_path_to_string(chain_home_dir),
                    "--chain-id",
                    &chain_id,
                    "init",
                    &chain_id,
                ],
            )
            .await
            .map_err(Bootstrap::runtime_error)?;

        Ok(())
    }
}
