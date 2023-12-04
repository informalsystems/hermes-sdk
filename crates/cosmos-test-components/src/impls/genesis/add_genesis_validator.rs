use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::traits::chain::types::amount::HasAmountType;

use crate::traits::fields::chain_command_path::HasChainCommandPath;
use crate::traits::genesis::add_genesis_validator::GenesisValidatorAdder;
use crate::traits::runtime::exec_command::CanExecCommand;
use crate::traits::runtime::types::file_path::HasFilePathType;

/**
   Implementation for adding genesis validator to Cosmos chains
   with Cosmos SDK version > v0.47.0. For legacy Cosmos chains,
   use `LegacyAddCosmosGenesisValidator` instead.
*/
pub struct AddCosmosGenesisValidator;

#[async_trait]
impl<Bootstrap, Runtime> GenesisValidatorAdder<Bootstrap> for AddCosmosGenesisValidator
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainIdType
        + HasAmountType
        + HasErrorType
        + HasChainCommandPath,
    Runtime: HasFilePathType + CanExecCommand,
{
    async fn add_genesis_validator(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        chain_id: &Bootstrap::ChainId,
        wallet_id: &str,
        stake_amount: &Bootstrap::Amount,
    ) -> Result<(), Bootstrap::Error> {
        bootstrap
            .runtime()
            .exec_command(
                "add genesis validator",
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
            .map_err(Bootstrap::runtime_error)?;

        Ok(())
    }
}
