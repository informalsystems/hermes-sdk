use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::bootstrap::traits::types::chain::HasChainType;
use ibc_test_components::chain::traits::types::amount::HasAmountType;

use crate::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use crate::bootstrap::traits::genesis::add_genesis_validator::GenesisValidatorAdder;
use ibc_test_components::runtime::traits::exec_command::CanExecCommand;
use ibc_test_components::runtime::traits::types::file_path::HasFilePathType;

/**
   Implementation for adding genesis validator to legacy Cosmos chains
   with Cosmos SDK version < v0.47.0.
   The chain command uses the `gentx` subcommand directly,
   without a `genesis` subcommand.
*/
pub struct LegacyAddCosmosGenesisValidator;

#[async_trait]
impl<Bootstrap, Runtime, Chain> GenesisValidatorAdder<Bootstrap> for LegacyAddCosmosGenesisValidator
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + HasErrorType
        + HasChainCommandPath,
    Runtime: HasFilePathType + CanExecCommand,
    Chain: HasChainIdType + HasAmountType,
{
    async fn add_genesis_validator(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        chain_id: &Chain::ChainId,
        wallet_id: &str,
        stake_amount: &Chain::Amount,
    ) -> Result<(), Bootstrap::Error> {
        bootstrap
            .runtime()
            .exec_command(
                bootstrap.chain_command_path(),
                &[
                    "--home",
                    &Runtime::file_path_to_string(chain_home_dir),
                    "gentx",
                    wallet_id,
                    "--keyring-backend",
                    "test",
                    "--chain-id",
                    &chain_id.to_string(),
                    &stake_amount.to_string(),
                ],
            )
            .await
            .map_err(Bootstrap::runtime_error)?;

        Ok(())
    }
}
