use cgp_core::Async;
use ibc_test_components::traits::chain::types::denom::DenomTypeProvider;

use crate::types::tests::denom::Denom;

pub struct ProvideIbcDenom;

impl<Chain> DenomTypeProvider<Chain> for ProvideIbcDenom
where
    Chain: Async,
{
    type Denom = Denom;
}
