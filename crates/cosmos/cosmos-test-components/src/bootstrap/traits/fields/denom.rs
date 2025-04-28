use core::marker::PhantomData;

use hermes_core::chain_type_components::traits::{DenomOf, HasDenomType};
use hermes_core::test_components::chain_driver::traits::HasChainType;
use hermes_prelude::*;

use crate::bootstrap::traits::HasChainGenesisConfigType;

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
