use cgp::prelude::*;

use crate::traits::types::amount::HasAmountType;
use crate::traits::types::quantity::HasQuantityType;

#[cgp_component {
  name: AmountQuantityGetterComponent,
  provider: AmountQuantityGetter,
  context: Chain,
}]
pub trait HasAmountQuantity: HasAmountType + HasQuantityType {
    fn amount_quantity(amount: &Self::Amount) -> &Self::Quantity;
}
