use crate::traits::chain::types::denom::HasDenomType;

pub trait HasDenom<const I: usize>: HasDenomType {
    fn denom(&self) -> &Self::Denom;
}
