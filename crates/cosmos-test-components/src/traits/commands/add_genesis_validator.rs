use cgp_core::prelude::*;
use ibc_test_components::traits::chain::types::amount::HasAmountType;

use crate::traits::types::file_path::HasFilePathType;

#[async_trait]
pub trait CanRunAddGenesisValidatorCommand: HasFilePathType + HasAmountType + HasErrorType {
    async fn run_add_genesis_validator_command(
        &self,
        chain_home_dir: &Self::FilePath,
        wallet_id: &str,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
