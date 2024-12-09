use core::marker::PhantomData;

use cgp::prelude::*;

use crate::chain::traits::types::denom::{DenomOf, HasDenomType};
use crate::chain_driver::traits::types::chain::HasChainType;

#[derive(Clone, Copy, Default)]
pub struct TransferDenom;

#[derive(Clone, Copy, Default)]
pub struct StakingDenom;

#[cgp_component {
  name: DenomGetterComponent,
  provider: DenomGetterAt,
  context: ChainDriver,
}]
pub trait HasDenomAt<DenomKind, I: Async>: HasChainType
where
    Self::Chain: HasDenomType,
{
    fn denom_at(&self, _kind: DenomKind, _index: PhantomData<I>) -> &DenomOf<Self::Chain>;
}
