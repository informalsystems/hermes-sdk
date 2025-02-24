use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::os::exec_command::CanExecCommand;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

use crate::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use crate::bootstrap::traits::initializers::init_chain_data::{
    ChainDataInitializer, ChainDataInitializerComponent,
};

pub struct InitCosmosChainData;

#[cgp_provider(ChainDataInitializerComponent)]
impl<Bootstrap, Runtime, Chain> ChainDataInitializer<Bootstrap> for InitCosmosChainData
where
    Bootstrap: HasChainType<Chain = Chain>
        + HasRuntime<Runtime = Runtime>
        + HasChainCommandPath
        + CanRaiseAsyncError<Runtime::Error>,
    Runtime: HasFilePathType + CanExecCommand,
    Chain: HasChainIdType,
{
    async fn init_chain_data(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        chain_id: &Chain::ChainId,
    ) -> Result<(), Bootstrap::Error> {
        let chain_id = chain_id.to_string();
        let chain_command_path = bootstrap.chain_command_path();

        bootstrap
            .runtime()
            .exec_command(
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
            .map_err(Bootstrap::raise_error)?;

        Ok(())
    }
}
