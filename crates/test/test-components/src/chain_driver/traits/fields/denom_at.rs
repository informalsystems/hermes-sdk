use cgp_core::prelude::*;

use crate::chain_driver::traits::types::denom::HasDenomType;
use crate::types::index::Index;

#[derive(Clone, Copy, Default)]
pub struct TransferDenom;

#[derive(Clone, Copy, Default)]
pub struct StakingDenom;

#[derive_component(DenomGetterComponent, DenomGetterAt<ChainDriver>)]
pub trait HasDenomAt<DenomKind, const I: usize>: HasDenomType {
    fn denom_at(&self, kind: DenomKind, index: Index<I>) -> &Self::Denom;
}
