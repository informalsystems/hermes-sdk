use cgp_core::prelude::*;
use ibc_test_components::traits::chain::types::address::HasAddressType;
use ibc_test_components::traits::chain::types::amount::HasAmountType;

#[async_trait]
pub trait CanAddGenesisBalance: HasAmountType + HasAddressType + HasErrorType {
    async fn add_genesis_balance(
        &self,
        address: &Self::Address,
        amounts: &[Self::Amount],
    ) -> Result<(), Self::Error>;
}
