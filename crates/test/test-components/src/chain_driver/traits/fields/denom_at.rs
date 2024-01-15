use crate::chain_driver::traits::types::denom::HasDenomType;

pub trait HasDenomAt<const I: usize>: HasDenomType {
    fn denom(&self) -> &Self::Denom;
}
