use ibc_test_components::chain::traits::types::amount::AmountTypeProvider;
use ibc_test_components::chain::traits::types::denom::HasDenomType;

use crate::types::amount::Amount;
use crate::types::denom::Denom;

pub struct ProvideIbcAmount;

impl<Chain> AmountTypeProvider<Chain> for ProvideIbcAmount
where
    Chain: HasDenomType<Denom = Denom>,
{
    type Amount = Amount;

    fn amount_denom(amount: &Self::Amount) -> &<Chain as HasDenomType>::Denom {
        &amount.denom
    }
}
