use hermes_chain_type_components::traits::{AmountOf, HasAmountType, HasDenomType};
use hermes_prelude::*;

use crate::chain_driver::traits::HasChainType;

#[cgp_component {
  provider: RandomAmountGenerator,
  context: Chain,
}]
#[async_trait]
pub trait CanGenerateRandomAmount: HasChainType
where
    Self::Chain: HasAmountType + HasDenomType,
{
    async fn random_amount(&self, min: usize, max: &AmountOf<Self::Chain>)
        -> AmountOf<Self::Chain>;

    async fn fixed_amount(
        &self,
        amount: usize,
        denom: &<Self::Chain as HasDenomType>::Denom,
    ) -> AmountOf<Self::Chain>;
}
