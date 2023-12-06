use cgp_core::prelude::*;
use eyre::{eyre, Report};
use ibc_relayer::keyring::{Secp256k1KeyPair, SigningKeyPair};
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::bootstrap::traits::types::chain::HasChainType;
use ibc_test_components::chain::traits::types::wallet::HasWalletType;
use serde_json as json;

use crate::traits::fields::chain_command_path::HasChainCommandPath;
use crate::traits::fields::hd_path::HasWalletHdPath;
use crate::traits::initializers::init_wallet::WalletInitializer;
use crate::types::wallet::CosmosTestWallet;
use ibc_test_components::runtime::traits::exec_command::CanExecCommand;
use ibc_test_components::runtime::traits::types::file_path::HasFilePathType;
use ibc_test_components::runtime::traits::write_file::CanWriteStringToFile;

pub struct InitCosmosTestWallet;

#[async_trait]
impl<Bootstrap, Runtime, Chain> WalletInitializer<Bootstrap> for InitCosmosTestWallet
where
    Bootstrap: HasErrorType
        + HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + HasWalletHdPath
        + HasChainCommandPath,
    Runtime: HasFilePathType + CanExecCommand + CanWriteStringToFile,
    Chain: HasWalletType<Wallet = CosmosTestWallet>,
    Bootstrap::Error: From<Report>,
{
    async fn initialize_wallet(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        wallet_id: &str,
    ) -> Result<Chain::Wallet, Bootstrap::Error> {
        let seed_content = bootstrap
            .runtime()
            .exec_command(
                bootstrap.chain_command_path(),
                &[
                    "--home",
                    &Runtime::file_path_to_string(chain_home_dir),
                    "keys",
                    "add",
                    wallet_id,
                    "--keyring-backend",
                    "test",
                    "--output",
                    "json",
                ],
            )
            .await
            .map_err(Bootstrap::runtime_error)?
            .stdout;

        let json_val: json::Value = json::from_str(&seed_content).map_err(Report::from)?;

        let wallet_address = json_val
            .get("address")
            .ok_or_else(|| eyre!("expect address string field to be present in json result"))?
            .as_str()
            .ok_or_else(|| eyre!("expect address string field to be present in json result"))?
            .to_string();

        // Write the wallet secret as a file so that a tester can use it during manual tests
        let seed_path = Runtime::join_file_path(
            chain_home_dir,
            &Runtime::file_path_from_string(&format!("{wallet_id}-seed.json")),
        );

        bootstrap
            .runtime()
            .write_string_to_file(&seed_path, &seed_content)
            .await
            .map_err(Bootstrap::runtime_error)?;

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
