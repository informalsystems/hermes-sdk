use cgp::prelude::*;
use hermes_chain_type_components::traits::types::amount::{AmountOf, HasAmountType};

use crate::chain::traits::types::denom::HasDenomType;
use crate::chain_driver::traits::types::chain::HasChainType;

#[cgp_component {
  provider: RandomAmountGenerator,
  context: Chain,
}]
#[async_trait]
pub trait CanGenerateRandomAmount: HasChainType
where
    Self::Chain: HasDenomType + HasAmountType,
{
    async fn random_amount(&self, min: usize, max: &AmountOf<Self::Chain>)
        -> AmountOf<Self::Chain>;
}
