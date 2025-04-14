use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::{DenomOf, HasDenomType};
use hermes_test_components::chain_driver::traits::HasChainType;

use crate::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;

pub struct DenomForStaking;

pub struct DenomForTransfer;

#[cgp_component {
    provider: GenesisDenomGetter,
    context: Bootstrap,
}]
pub trait HasGenesisDenom<Label>: HasChainGenesisConfigType + HasChainType
where
    Self::Chain: HasDenomType,
{
    fn genesis_denom(
        chain_config: &Self::ChainGenesisConfig,
        _label: PhantomData<Label>,
    ) -> &DenomOf<Self::Chain>;
}

#[cgp_getter {
    name: DenomPrefixGetterComponent<Label>,
    provider: DenomPrefixGetter,
}]
pub trait HasDenomPrefix<Label>: Async {
    fn denom_prefix(&self, label: PhantomData<Label>) -> &str;
}
