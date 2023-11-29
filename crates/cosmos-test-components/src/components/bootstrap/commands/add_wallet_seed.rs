use cgp_core::prelude::*;

use crate::traits::bootstrap::commands::add_wallet_seed::AddWalletSeedCommandRunner;
use crate::traits::bootstrap::hd_path::HasWalletHdPath;
use crate::traits::bootstrap::write_file::CanWriteFile;
use crate::traits::chain_command_path::HasChainCommandPath;
use crate::traits::exec_command::CanExecCommand;
use crate::traits::file_path::HasFilePathType;

pub struct AddCosmosTestWalletSeed;

#[async_trait]
impl<Bootstrap> AddWalletSeedCommandRunner<Bootstrap> for AddCosmosTestWalletSeed
where
    Bootstrap: HasErrorType
        + HasFilePathType
        + CanExecCommand
        + HasChainCommandPath
        + CanWriteFile
        + HasWalletHdPath,
{
    async fn run_add_wallet_seed_command(
        bootstrap: &Bootstrap,
        chain_home_dir: &Bootstrap::FilePath,
        wallet_id: &str,
    ) -> Result<String, Bootstrap::Error> {
        let seed_content = bootstrap
            .exec_command(
                "add wallet",
                bootstrap.chain_command_path(),
                &[
                    "--home",
                    &Bootstrap::file_path_to_string(chain_home_dir),
                    "keys",
                    "add",
                    wallet_id,
                    "--keyring-backend",
                    "test",
                    "--output",
                    "json",
                ],
            )
            .await?
            .stdout;

        Ok(seed_content)
    }
}
