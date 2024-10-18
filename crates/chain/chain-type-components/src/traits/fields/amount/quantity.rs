use cgp::prelude::*;

use crate::traits::types::amount::HasAmountType;
use crate::traits::types::quantity::HasQuantityType;

#[derive_component(AmountQuantityGetterComponent, AmountQuantityGetter<Chain>)]
pub trait HasAmountQuantity: HasAmountType + HasQuantityType {
    fn amount_quantity(amount: &Self::Amount) -> &Self::Quantity;
}
