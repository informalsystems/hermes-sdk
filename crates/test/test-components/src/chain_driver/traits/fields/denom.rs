use core::marker::PhantomData;

use cgp::prelude::*;

use crate::chain::traits::types::denom::{DenomOf, HasDenomType};
use crate::chain_driver::traits::types::chain::HasChainType;

#[derive(Clone, Copy, Default)]
pub struct TransferDenom;

#[derive(Clone, Copy, Default)]
pub struct StakingDenom;

#[cgp_getter {
    name: DenomGetterComponent<DenomKind>,
    provider: DenomGetter,
}]
pub trait HasDenom<DenomKind>: HasChainType<Chain: HasDenomType> {
    fn denom(&self, _phantom: PhantomData<DenomKind>) -> &DenomOf<Self::Chain>;
}
