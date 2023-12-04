use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;

use crate::traits::fields::chain_command_path::HasChainCommandPath;
use crate::traits::genesis::collect_gentxs::GenesisTransactionsCollector;
use crate::traits::runtime::exec_command::CanExecCommand;
use crate::traits::runtime::types::file_path::HasFilePathType;

pub struct LegacyCollectCosmosGentxs;

#[async_trait]
impl<Bootstrap, Runtime> GenesisTransactionsCollector<Bootstrap> for LegacyCollectCosmosGentxs
where
    Bootstrap: HasRuntime<Runtime = Runtime> + HasChainIdType + HasErrorType + HasChainCommandPath,
    Runtime: HasFilePathType + CanExecCommand,
{
    async fn collect_genesis_transactions(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
    ) -> Result<(), Bootstrap::Error> {
        bootstrap
            .runtime()
            .exec_command(
                "collect gentxs",
                bootstrap.chain_command_path(),
                &[
                    "--home",
                    &Runtime::file_path_to_string(chain_home_dir),
                    "collect-gentxs",
                ],
            )
            .await
            .map_err(Bootstrap::runtime_error)?;

        Ok(())
    }
}
