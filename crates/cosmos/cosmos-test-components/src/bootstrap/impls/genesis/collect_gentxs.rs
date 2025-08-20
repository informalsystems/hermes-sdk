use hermes_core::runtime_components::traits::{CanExecCommand, HasFilePathType, HasRuntime};
use hermes_prelude::*;
use hermes_test_components::chain_driver::traits::HasChainCommandPath;

use crate::bootstrap::traits::{
    GenesisTransactionsCollector, GenesisTransactionsCollectorComponent,
};

pub struct CollectCosmosGentxs;

#[cgp_provider(GenesisTransactionsCollectorComponent)]
impl<Bootstrap, Runtime> GenesisTransactionsCollector<Bootstrap> for CollectCosmosGentxs
where
    Bootstrap:
        HasRuntime<Runtime = Runtime> + CanRaiseAsyncError<Runtime::Error> + HasChainCommandPath,
    Runtime: HasFilePathType + CanExecCommand,
{
    async fn collect_genesis_transactions(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
    ) -> Result<(), Bootstrap::Error> {
        bootstrap
            .runtime()
            .exec_command(
                bootstrap.chain_command_path(),
                &[
                    "--home",
                    &Runtime::file_path_to_string(chain_home_dir),
                    "genesis",
                    "collect-gentxs",
                ],
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(())
    }
}
