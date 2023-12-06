use cgp_core::prelude::*;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::bootstrap::traits::types::chain::HasChainType;
use ibc_test_components::chain::traits::types::address::HasAddressType;
use ibc_test_components::chain::traits::types::amount::HasAmountType;

use crate::traits::fields::chain_command_path::HasChainCommandPath;
use crate::traits::genesis::add_genesis_account::GenesisAccountAdder;
use ibc_test_components::runtime::traits::exec_command::CanExecCommand;
use ibc_test_components::runtime::traits::types::file_path::HasFilePathType;

/**
   Implementation for adding genesis account to Cosmos chains
   with Cosmos SDK version > v0.47.0. For legacy Cosmos chains,
   use `LegacyAddCosmosGenesisAccount` instead.
*/
pub struct AddCosmosGenesisAccount;

#[async_trait]
impl<Bootstrap, Runtime, Chain> GenesisAccountAdder<Bootstrap> for AddCosmosGenesisAccount
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + HasAmountType
        + HasAddressType
        + HasErrorType
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
            .map_err(Bootstrap::runtime_error)?;

        Ok(())
    }
}
