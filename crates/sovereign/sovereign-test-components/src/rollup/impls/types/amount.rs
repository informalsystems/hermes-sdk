use hermes_test_components::chain::traits::types::amount::ProvideAmountType;
use hermes_test_components::chain::traits::types::denom::HasDenomType;

use crate::types::amount::SovereignAmount;

pub struct ProvideSovereignAmountType;

impl<Rollup> ProvideAmountType<Rollup> for ProvideSovereignAmountType
where
    Rollup: HasDenomType<Denom = String>,
{
    type Amount = SovereignAmount;

    fn amount_denom(amount: &SovereignAmount) -> &String {
        &amount.denom
    }
}
