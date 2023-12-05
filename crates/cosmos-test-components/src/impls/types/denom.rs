use cgp_core::Async;
use ibc_test_components::chain::traits::types::denom::DenomTypeProvider;

use crate::types::denom::Denom;

pub struct ProvideIbcDenom;

impl<Chain> DenomTypeProvider<Chain> for ProvideIbcDenom
where
    Chain: Async,
{
    type Denom = Denom;
}
