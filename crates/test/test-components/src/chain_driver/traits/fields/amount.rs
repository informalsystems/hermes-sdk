use hermes_chain_type_components::traits::{AmountOf, HasAmountType};
use hermes_prelude::*;

use crate::chain_driver::traits::HasChainType;

#[cgp_component {
  provider: RandomAmountGenerator,
  context: Chain,
}]
#[async_trait]
pub trait CanGenerateRandomAmount: HasChainType
where
    Self::Chain: HasAmountType,
{
    async fn random_amount(&self, min: usize, max: &AmountOf<Self::Chain>)
        -> AmountOf<Self::Chain>;
}
