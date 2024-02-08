use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::amount::HasAmountType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;
use hermes_test_components::runtime::traits::exec_command::CanExecCommand;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;

use crate::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use crate::bootstrap::traits::genesis::add_genesis_validator::GenesisValidatorAdder;

/**
   Implementation for adding genesis validator to Cosmos chains
   with Cosmos SDK version > v0.47.0. For legacy Cosmos chains,
   use `LegacyAddCosmosGenesisValidator` instead.
*/
pub struct AddCosmosGenesisValidator;

#[async_trait]
impl<Bootstrap, Runtime, Chain, ChainDriver> GenesisValidatorAdder<Bootstrap>
    for AddCosmosGenesisValidator
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainDriverType<Chain = Chain, ChainDriver = ChainDriver>
        + CanRaiseError<Runtime::Error>
        + HasChainCommandPath,
    Runtime: HasFilePathType + CanExecCommand,
    Chain: HasChainIdType,
    ChainDriver: HasAmountType,
{
    async fn add_genesis_validator(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        chain_id: &Chain::ChainId,
        wallet_id: &str,
        stake_amount: &ChainDriver::Amount,
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
