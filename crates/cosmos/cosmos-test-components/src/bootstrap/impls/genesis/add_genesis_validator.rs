use cgp::prelude::*;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::os::exec_command::CanExecCommand;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

use crate::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use crate::bootstrap::traits::genesis::add_genesis_validator::{
    GenesisValidatorAdder, GenesisValidatorAdderComponent,
};

/**
   Implementation for adding genesis validator to Cosmos chains
   with Cosmos SDK version > v0.47.0. For legacy Cosmos chains,
   use `LegacyAddCosmosGenesisValidator` instead.
*/
pub struct AddCosmosGenesisValidator;

#[cgp_provider(GenesisValidatorAdderComponent)]
impl<Bootstrap, Runtime, Chain> GenesisValidatorAdder<Bootstrap> for AddCosmosGenesisValidator
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + CanRaiseAsyncError<Runtime::Error>
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
                    "genesis",
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
