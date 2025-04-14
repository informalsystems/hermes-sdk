use cgp::prelude::*;

use crate::traits::{HasAmountType, HasDenomType, HasQuantityType};

#[cgp_component {
  provider: AmountBuilder,
  context: Chain,
}]
pub trait CanBuildAmount: HasDenomType + HasQuantityType + HasAmountType {
    fn build_amount(denom: &Self::Denom, quantity: &Self::Quantity) -> Self::Amount;
}
