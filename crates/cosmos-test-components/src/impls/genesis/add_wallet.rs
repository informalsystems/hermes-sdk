use cgp_core::prelude::*;
use eyre::{eyre, Report};
use ibc_relayer::keyring::{Secp256k1KeyPair, SigningKeyPair};
use ibc_test_components::traits::chain::types::wallet::HasWalletType;
use serde_json as json;

use crate::traits::commands::add_wallet_seed::CanRunAddWalletSeedCommand;
use crate::traits::fields::chain_command_path::HasChainCommandPath;
use crate::traits::fields::hd_path::HasWalletHdPath;
use crate::traits::genesis::add_wallet::GenesisWalletAdder;
use crate::traits::io::exec_command::CanExecCommand;
use crate::traits::io::write_file::CanWriteStringToFile;
use crate::traits::types::file_path::HasFilePathType;
use crate::types::wallet::CosmosTestWallet;

pub struct AddTestWalletWithCosmosSeed;

#[async_trait]
impl<Bootstrap> GenesisWalletAdder<Bootstrap> for AddTestWalletWithCosmosSeed
where
    Bootstrap: HasErrorType
        + HasWalletType<Wallet = CosmosTestWallet>
        + CanRunAddWalletSeedCommand
        + HasFilePathType
        + CanWriteStringToFile
        + HasWalletHdPath
        + CanExecCommand
        + HasChainCommandPath,
    Bootstrap::Error: From<Report>,
{
    async fn add_genesis_wallet(
        bootstrap: &Bootstrap,
        chain_home_dir: &Bootstrap::FilePath,
        wallet_id: &str,
    ) -> Result<Bootstrap::Wallet, Bootstrap::Error> {
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

        let json_val: json::Value = json::from_str(&seed_content).map_err(Report::from)?;

        let wallet_address = json_val
            .get("address")
            .ok_or_else(|| eyre!("expect address string field to be present in json result"))?
            .as_str()
            .ok_or_else(|| eyre!("expect address string field to be present in json result"))?
            .to_string();

        // Write the wallet secret as a file so that a tester can use it during manual tests
        let seed_path = Bootstrap::join_file_path(
            chain_home_dir,
            &Bootstrap::file_path_from_string(&format!("{wallet_id}-seed.json")),
        );
        bootstrap
            .write_string_to_file(&seed_path, &seed_content)
            .await?;

        let hd_path = bootstrap.wallet_hd_path();

        let keypair =
            Secp256k1KeyPair::from_seed_file(&seed_content, &hd_path).map_err(Report::from)?;

        let wallet = CosmosTestWallet {
            id: wallet_id.to_owned(),
            address: wallet_address,
            keypair,
        };

        Ok(wallet)
    }
}
