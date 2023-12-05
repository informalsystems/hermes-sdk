use crate::chain::traits::types::denom::HasDenomType;

pub trait HasDenom<const I: usize>: HasDenomType {
    fn denom(&self) -> &Self::Denom;
}
