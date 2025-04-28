use hermes_prelude::*;

use crate::traits::{HasAmountType, HasQuantityType};

#[cgp_component {
  provider: AmountQuantityGetter,
  context: Chain,
}]
pub trait HasAmountQuantity: HasAmountType + HasQuantityType {
    fn amount_quantity(amount: &Self::Amount) -> &Self::Quantity;
}
