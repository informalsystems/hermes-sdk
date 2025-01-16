use cgp::core::error::CanRaiseAsyncError;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::os::exec_command::CanExecCommand;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

use crate::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use crate::bootstrap::traits::genesis::add_genesis_account::GenesisAccountAdder;

/**
   Implementation for adding genesis account to Cosmos chains
   with Cosmos SDK version > v0.47.0. For legacy Cosmos chains,
   use `LegacyAddCosmosGenesisAccount` instead.
*/
pub struct AddCosmosGenesisAccount;

impl<Bootstrap, Runtime, Chain> GenesisAccountAdder<Bootstrap> for AddCosmosGenesisAccount
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + CanRaiseAsyncError<Runtime::Error>
        + HasChainCommandPath,
    Runtime: HasFilePathType + CanExecCommand,
    Chain: HasAmountType + HasAddressType,
{
    async fn add_genesis_account(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        address: &Chain::Address,
        amounts: &[Chain::Amount],
    ) -> Result<(), Bootstrap::Error> {
        let amounts_string = itertools::join(amounts, ",");

        bootstrap
            .runtime()
            .exec_command(
                bootstrap.chain_command_path(),
                &[
                    "--home",
                    &Runtime::file_path_to_string(chain_home_dir),
                    "genesis",
                    "add-genesis-account",
                    &address.to_string(),
                    &amounts_string,
                ],
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(())
    }
}
