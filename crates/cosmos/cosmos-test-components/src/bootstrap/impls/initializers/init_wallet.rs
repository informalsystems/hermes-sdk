use core::marker::PhantomData;

use hermes_core::runtime_components::traits::{
    CanExecCommand, CanWriteStringToFile, ExecOutput, HasFilePathType, HasRuntime,
};
use hermes_core::test_components::chain::traits::HasWalletType;
use hermes_core::test_components::chain_driver::traits::HasChainType;
use hermes_cosmos_chain_components::types::Secp256k1KeyPair;
use hermes_prelude::*;
use serde_json as json;

use crate::bootstrap::traits::{
    HasChainCommandPath, HasWalletHdPath, WalletInitializer, WalletInitializerComponent,
};
use crate::chain::types::CosmosTestWallet;

pub struct InitCosmosTestWallet<OutputGetter>(pub PhantomData<OutputGetter>);

pub trait ExecOutputGetter: Async {
    fn get_exec_output(output: ExecOutput) -> String;
}

pub struct GetStdOut;

pub struct GetStdOutOrElseStdErr;

impl ExecOutputGetter for GetStdOut {
    fn get_exec_output(output: ExecOutput) -> String {
        output.stdout
    }
}

impl ExecOutputGetter for GetStdOutOrElseStdErr {
    fn get_exec_output(output: ExecOutput) -> String {
        if !output.stdout.is_empty() {
            output.stdout
        } else {
            output.stderr
        }
    }
}

#[cgp_provider(WalletInitializerComponent)]
impl<Bootstrap, Runtime, Chain, OutputGetter> WalletInitializer<Bootstrap>
    for InitCosmosTestWallet<OutputGetter>
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + HasWalletHdPath
        + HasChainCommandPath
        + CanRaiseAsyncError<String>
        + CanRaiseAsyncError<Runtime::Error>
        + CanRaiseAsyncError<&'static str>
        + CanRaiseAsyncError<json::Error>,
    Runtime: HasFilePathType + CanExecCommand + CanWriteStringToFile,
    Chain: HasWalletType<Wallet = CosmosTestWallet>,
    OutputGetter: ExecOutputGetter,
{
    async fn initialize_wallet(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        wallet_id: &str,
    ) -> Result<Chain::Wallet, Bootstrap::Error> {
        let add_wallet_output = bootstrap
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
            .map_err(Bootstrap::raise_error)?;

        let seed_content = OutputGetter::get_exec_output(add_wallet_output);

        let json_val: json::Value =
            json::from_str(&seed_content).map_err(Bootstrap::raise_error)?;

        let wallet_address = json_val
            .get("address")
            .ok_or_else(|| {
                Bootstrap::raise_error("expect address string field to be present in json result")
            })?
            .as_str()
            .ok_or_else(|| {
                Bootstrap::raise_error("expect address string field to be present in json result")
            })?
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
            .map_err(Bootstrap::raise_error)?;

        let hd_path = bootstrap.wallet_hd_path();

        let keypair = Secp256k1KeyPair::from_seed_file(&seed_content, &hd_path)
            .map_err(Bootstrap::raise_error)?;

        let wallet = CosmosTestWallet {
            id: wallet_id.to_owned(),
            address: wallet_address,
            keypair,
        };

        Ok(wallet)
    }
}
