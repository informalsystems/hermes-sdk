use cgp_core::prelude::*;
use ibc_test_components::traits::chain::types::address::HasAddressType;
use ibc_test_components::traits::chain::types::amount::HasAmountType;

use crate::traits::fields::chain_command_path::HasChainCommandPath;
use crate::traits::genesis::add_genesis_account::GenesisAccountAdder;
use crate::traits::io::exec_command::CanExecCommand;
use crate::traits::types::file_path::HasFilePathType;

pub struct LegacyAddCosmosGenesisAccount;

#[async_trait]
impl<Bootstrap> GenesisAccountAdder<Bootstrap> for LegacyAddCosmosGenesisAccount
where
    Bootstrap: HasFilePathType
        + HasAmountType
        + HasAddressType
        + HasErrorType
        + CanExecCommand
        + HasChainCommandPath,
{
    async fn add_genesis_account(
        bootstrap: &Bootstrap,
        chain_home_dir: &Bootstrap::FilePath,
        address: &Bootstrap::Address,
        amounts: &[Bootstrap::Amount],
    ) -> Result<(), Bootstrap::Error> {
        let amounts_string = itertools::join(amounts, ",");

        bootstrap
            .exec_command(
                "add wallet",
                bootstrap.chain_command_path(),
                &[
                    "--home",
                    &Bootstrap::file_path_to_string(chain_home_dir),
                    "add-genesis-account",
                    &address.to_string(),
                    &amounts_string,
                ],
            )
            .await?;

        Ok(())
    }
}
