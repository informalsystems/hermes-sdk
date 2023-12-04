use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;

use crate::traits::fields::chain_command_path::HasChainCommandPath;
use crate::traits::initializers::init_chain_data::ChainDataInitializer;
use crate::traits::io::exec_command::CanExecCommand;
use crate::traits::types::io::file_path::HasFilePathType;

pub struct InitCosmosChainData;

#[async_trait]
impl<Bootstrap> ChainDataInitializer<Bootstrap> for InitCosmosChainData
where
    Bootstrap: HasChainIdType + HasFilePathType + HasChainCommandPath + CanExecCommand,
{
    async fn init_chain_data(
        bootstrap: &Bootstrap,
        chain_home_dir: &Bootstrap::FilePath,
        chain_id: &Bootstrap::ChainId,
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
