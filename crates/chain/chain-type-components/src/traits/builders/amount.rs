use cgp::prelude::*;

use crate::traits::types::amount::HasAmountType;
use crate::traits::types::denom::HasDenomType;
use crate::traits::types::quantity::HasQuantityType;

#[derive_component(AmountBuilderComponent, AmountBuilder<Chain>)]
pub trait CanBuildAmount: HasDenomType + HasQuantityType + HasAmountType {
    fn build_amount(denom: &Self::Denom, quantity: &Self::Quantity) -> Self::Amount;
}
