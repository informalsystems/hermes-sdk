use std::path::Path;

use cgp_core::prelude::*;
use eyre::{eyre, Report};
use ibc_test_components::traits::chain::types::wallet::HasWalletType;
use serde_json as json;

use crate::traits::bootstrap::commands::add_wallet::AddWalletCommandRunner;
use crate::traits::chain_command_path::HasChainCommandPath;
use crate::traits::exec_command::CanExecCommand;

pub struct AddCosmosTestWallet;

#[async_trait]
impl<Bootstrap> AddWalletCommandRunner<Bootstrap> for AddCosmosTestWallet
where
    Bootstrap: HasErrorType + HasWalletType + CanExecCommand + HasChainCommandPath,
    Bootstrap::Error: From<Report>,
{
    async fn run_add_wallet_command(
        bootstrap: &Bootstrap,
        chain_home_dir: &Path,
        wallet_id: &str,
    ) -> Result<Bootstrap::Wallet, Bootstrap::Error> {
        let seed_content = bootstrap
            .exec_command(
                "add wallet",
                bootstrap.chain_command_path(),
                &[
                    "--home",
                    &chain_home_dir.to_string_lossy(),
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

        let json_val: json::Value = json::from_str(&seed_content).map_err(Report::from)?;

        let _wallet_address = json_val
            .get("address")
            .ok_or_else(|| eyre!("expect address string field to be present in json result"))?
            .as_str()
            .ok_or_else(|| eyre!("expect address string field to be present in json result"))?
            .to_string();

        todo!()
    }
}
