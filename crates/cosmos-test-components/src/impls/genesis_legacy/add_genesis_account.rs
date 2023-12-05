use cgp_core::prelude::*;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::chain::traits::types::address::HasAddressType;
use ibc_test_components::chain::traits::types::amount::HasAmountType;

use crate::traits::fields::chain_command_path::HasChainCommandPath;
use crate::traits::genesis::add_genesis_account::GenesisAccountAdder;
use ibc_test_components::runtime::traits::exec_command::CanExecCommand;
use ibc_test_components::runtime::traits::types::file_path::HasFilePathType;

/**
   Implementation for adding genesis account to legacy Cosmos chains
   with Cosmos SDK version < v0.47.0.
   The chain command uses the `add-genesis-account` subcommand directly,
   without a `genesis` subcommand.
*/
pub struct LegacyAddCosmosGenesisAccount;

#[async_trait]
impl<Bootstrap, Runtime> GenesisAccountAdder<Bootstrap> for LegacyAddCosmosGenesisAccount
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
                "add genesis account legacy",
                bootstrap.chain_command_path(),
                &[
                    "--home",
                    &Runtime::file_path_to_string(chain_home_dir),
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
