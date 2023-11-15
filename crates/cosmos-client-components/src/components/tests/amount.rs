use ibc_test_components::traits::chain::types::amount::AmountTypeProvider;
use ibc_test_components::traits::chain::types::denom::HasDenomType;

use crate::types::tests::amount::Amount;
use crate::types::tests::denom::Denom;

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
