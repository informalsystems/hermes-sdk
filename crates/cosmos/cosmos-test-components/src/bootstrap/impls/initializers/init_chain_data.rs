use cgp::prelude::*;
use hermes_core::relayer_components::chain::traits::HasChainIdType;
use hermes_core::runtime_components::traits::{CanExecCommand, HasFilePathType, HasRuntime};
use hermes_core::test_components::chain_driver::traits::HasChainType;

use crate::bootstrap::traits::{
    ChainDataInitializer, ChainDataInitializerComponent, HasChainCommandPath,
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
