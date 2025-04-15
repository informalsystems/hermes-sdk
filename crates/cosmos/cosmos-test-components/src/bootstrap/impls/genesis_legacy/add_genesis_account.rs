use cgp::prelude::*;
use hermes_core::chain_type_components::traits::{HasAddressType, HasAmountType};
use hermes_core::runtime_components::traits::{CanExecCommand, HasFilePathType, HasRuntime};
use hermes_core::test_components::chain_driver::traits::HasChainType;

use crate::bootstrap::traits::{
    GenesisAccountAdder, GenesisAccountAdderComponent, HasChainCommandPath,
};

/**
   Implementation for adding genesis account to legacy Cosmos chains
   with Cosmos SDK version < v0.47.0.
   The chain command uses the `add-genesis-account` subcommand directly,
   without a `genesis` subcommand.
*/
pub struct LegacyAddCosmosGenesisAccount;

#[cgp_provider(GenesisAccountAdderComponent)]
impl<Bootstrap, Runtime, Chain> GenesisAccountAdder<Bootstrap> for LegacyAddCosmosGenesisAccount
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + HasAsyncErrorType
        + HasChainCommandPath
        + CanRaiseAsyncError<Runtime::Error>,
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
