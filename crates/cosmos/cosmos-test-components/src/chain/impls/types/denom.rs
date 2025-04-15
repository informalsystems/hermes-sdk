use cgp::prelude::*;
use hermes_core::chain_type_components::traits::{DenomTypeComponent, ProvideDenomType};

use crate::chain::types::Denom;

pub struct ProvideIbcDenom;

#[cgp_provider(DenomTypeComponent)]
impl<Chain> ProvideDenomType<Chain> for ProvideIbcDenom
where
    Chain: Async,
{
    type Denom = Denom;
}
