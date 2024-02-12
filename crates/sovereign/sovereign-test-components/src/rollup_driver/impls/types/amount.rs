use hermes_test_components::chain_driver::traits::types::amount::ProvideAmountType;
use hermes_test_components::chain_driver::traits::types::denom::HasDenomType;

use crate::types::amount::SovereignAmount;

pub struct ProvideSovereignAmountType;

impl<RollupDriver> ProvideAmountType<RollupDriver> for ProvideSovereignAmountType
where
    RollupDriver: HasDenomType<Denom = String>,
{
    type Amount = SovereignAmount;

    fn amount_denom(amount: &SovereignAmount) -> &String {
        &amount.denom
    }
}
