use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;

use crate::traits::fields::chain_command_path::HasChainCommandPath;
use crate::traits::genesis::collect_gentxs::GentxsCollector;
use crate::traits::io::exec_command::CanExecCommand;
use crate::traits::types::file_path::HasFilePathType;

pub struct CollectCosmosGentxs;

#[async_trait]
impl<Bootstrap> GentxsCollector<Bootstrap> for CollectCosmosGentxs
where
    Bootstrap:
        HasFilePathType + HasChainIdType + HasErrorType + HasChainCommandPath + CanExecCommand,
{
    async fn collect_gentxs(
        bootstrap: &Bootstrap,
        chain_home_dir: &Bootstrap::FilePath,
    ) -> Result<(), Bootstrap::Error> {
        bootstrap
            .exec_command(
                "collect gentxs",
                bootstrap.chain_command_path(),
                &[
                    "--home",
                    &Bootstrap::file_path_to_string(chain_home_dir),
                    "genesis",
                    "collect-gentxs",
                ],
            )
            .await?;

        Ok(())
    }
}
