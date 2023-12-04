use cgp_core::prelude::*;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::traits::chain::types::address::HasAddressType;
use ibc_test_components::traits::chain::types::amount::HasAmountType;

use crate::traits::fields::chain_command_path::HasChainCommandPath;
use crate::traits::genesis::add_genesis_account::GenesisAccountAdder;
use crate::traits::runtime::exec_command::CanExecCommand;
use crate::traits::runtime::types::file_path::HasFilePathType;

/**
   Implementation for adding genesis account to Cosmos chains
   with Cosmos SDK version > v0.47.0. For legacy Cosmos chains,
   use `LegacyAddCosmosGenesisAccount` instead.
*/
pub struct AddCosmosGenesisAccount;

#[async_trait]
impl<Bootstrap, Runtime> GenesisAccountAdder<Bootstrap> for AddCosmosGenesisAccount
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasAmountType
        + HasAddressType
        + HasErrorType
        + HasChainCommandPath,
    Runtime: HasFilePathType + CanExecCommand,
{
    async fn add_genesis_account(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        address: &Bootstrap::Address,
        amounts: &[Bootstrap::Amount],
    ) -> Result<(), Bootstrap::Error> {
        let amounts_string = itertools::join(amounts, ",");

        bootstrap
            .runtime()
            .exec_command(
                "add genesis account",
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
            .map_err(Bootstrap::runtime_error)?;

        Ok(())
    }
}
