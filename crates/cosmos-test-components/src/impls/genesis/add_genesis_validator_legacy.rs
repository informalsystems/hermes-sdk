use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use ibc_test_components::traits::chain::types::amount::HasAmountType;

use crate::traits::fields::chain_command_path::HasChainCommandPath;
use crate::traits::genesis::add_genesis_validator::GenesisValidatorAdder;
use crate::traits::io::exec_command::CanExecCommand;
use crate::traits::types::file_path::HasFilePathType;

/**
   Implementation for adding genesis validator to legacy Cosmos chains
   with Cosmos SDK version < v0.47.0.
   The chain command uses the `gentx` subcommand directly,
   without a `genesis` subcommand.
*/
pub struct LegacyAddCosmosGenesisValidator;

#[async_trait]
impl<Bootstrap> GenesisValidatorAdder<Bootstrap> for LegacyAddCosmosGenesisValidator
where
    Bootstrap: HasChainIdType
        + HasFilePathType
        + HasAmountType
        + HasErrorType
        + CanExecCommand
        + HasChainCommandPath,
{
    async fn add_genesis_validator(
        bootstrap: &Bootstrap,
        chain_home_dir: &Bootstrap::FilePath,
        chain_id: &Bootstrap::ChainId,
        wallet_id: &str,
        stake_amount: &Bootstrap::Amount,
    ) -> Result<(), Bootstrap::Error> {
        bootstrap
            .exec_command(
                "add genesis validator",
                bootstrap.chain_command_path(),
                &[
                    "--home",
                    &Bootstrap::file_path_to_string(chain_home_dir),
                    "gentx",
                    wallet_id,
                    "--keyring-backend",
                    "test",
                    "--chain-id",
                    &chain_id.to_string(),
                    &stake_amount.to_string(),
                ],
            )
            .await?;

        Ok(())
    }
}
