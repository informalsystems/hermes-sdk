use hermes_core::chain_type_components::traits::HasAmountType;
use hermes_core::relayer_components::chain::traits::HasChainIdType;
use hermes_core::runtime_components::traits::{CanExecCommand, HasFilePathType, HasRuntime};
use hermes_core::test_components::chain_driver::traits::HasChainType;
use hermes_prelude::*;

use crate::bootstrap::traits::{
    GenesisValidatorAdder, GenesisValidatorAdderComponent, HasChainCommandPath,
};

/**
   Implementation for adding genesis validator to legacy Cosmos chains
   with Cosmos SDK version < v0.47.0.
   The chain command uses the `gentx` subcommand directly,
   without a `genesis` subcommand.
*/
pub struct LegacyAddCosmosGenesisValidator;

#[cgp_provider(GenesisValidatorAdderComponent)]
impl<Bootstrap, Runtime, Chain> GenesisValidatorAdder<Bootstrap> for LegacyAddCosmosGenesisValidator
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + HasChainCommandPath
        + CanRaiseAsyncError<Runtime::Error>,
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
            .map_err(Bootstrap::raise_error)?;

        Ok(())
    }
}
