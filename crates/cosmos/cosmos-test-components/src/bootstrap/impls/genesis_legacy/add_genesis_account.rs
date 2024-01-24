use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::address::HasAddressType;
use hermes_test_components::chain_driver::traits::types::amount::HasAmountType;

use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::exec_command::CanExecCommand;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;

use crate::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use crate::bootstrap::traits::genesis::add_genesis_account::GenesisAccountAdder;

/**
   Implementation for adding genesis account to legacy Cosmos chains
   with Cosmos SDK version < v0.47.0.
   The chain command uses the `add-genesis-account` subcommand directly,
   without a `genesis` subcommand.
*/
pub struct LegacyAddCosmosGenesisAccount;

#[async_trait]
impl<Bootstrap, Runtime, ChainDriver> GenesisAccountAdder<Bootstrap>
    for LegacyAddCosmosGenesisAccount
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainDriverType<ChainDriver = ChainDriver>
        + HasErrorType
        + HasChainCommandPath
        + CanRaiseError<Runtime::Error>,
    Runtime: HasFilePathType + CanExecCommand,
    ChainDriver: HasAmountType + HasAddressType,
{
    async fn add_genesis_account(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        address: &ChainDriver::Address,
        amounts: &[ChainDriver::Amount],
    ) -> Result<(), Bootstrap::Error> {
        let amounts_string = itertools::join(amounts, ",");

        bootstrap
            .runtime()
            .exec_command(
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
            .map_err(Bootstrap::raise_error)?;

        Ok(())
    }
}
