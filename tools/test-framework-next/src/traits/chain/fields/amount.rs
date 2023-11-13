use crate::traits::chain::types::amount::HasAmountType;

pub trait CanGenerateRandomAmount: HasAmountType {
    fn random_amount(min: usize, max: usize) -> Self::Amount;
}
