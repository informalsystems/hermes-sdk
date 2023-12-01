use cgp_core::prelude::*;
use ibc_test_components::traits::chain::types::amount::HasAmountType;

use crate::traits::types::file_path::HasFilePathType;

#[async_trait]
pub trait CanAddGenesisValidator: HasFilePathType + HasAmountType + HasErrorType {
    async fn add_genesis_validator(
        &self,
        chain_home_dir: &Self::FilePath,
        wallet_id: &str,
        amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
