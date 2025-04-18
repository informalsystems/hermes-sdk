use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::os::exec_command::CanExecCommand;
use hermes_runtime_components::traits::runtime::HasRuntime;

use crate::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use crate::bootstrap::traits::genesis::collect_gentxs::{
    GenesisTransactionsCollector, GenesisTransactionsCollectorComponent,
};

pub struct LegacyCollectCosmosGentxs;

#[cgp_provider(GenesisTransactionsCollectorComponent)]
impl<Bootstrap, Runtime> GenesisTransactionsCollector<Bootstrap> for LegacyCollectCosmosGentxs
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasAsyncErrorType
        + HasChainCommandPath
        + CanRaiseAsyncError<Runtime::Error>,
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
                    "collect-gentxs",
                ],
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(())
    }
}
