use cgp_core::prelude::*;

use crate::chain::traits::types::denom::{DenomOf, HasDenomType};
use crate::chain_driver::traits::types::chain::HasChainType;
use crate::types::index::Index;

#[derive(Clone, Copy, Default)]
pub struct TransferDenom;

#[derive(Clone, Copy, Default)]
pub struct StakingDenom;

#[derive_component(DenomGetterComponent, DenomGetterAt<ChainDriver>)]
pub trait HasDenomAt<DenomKind, const I: usize>: HasChainType
where
    Self::Chain: HasDenomType,
{
    fn denom_at(&self, kind: DenomKind, index: Index<I>) -> &DenomOf<Self::Chain>;
}
