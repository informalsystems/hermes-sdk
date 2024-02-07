use cgp_core::prelude::*;
use hermes_test_components::chain_driver::traits::types::denom::{DenomOf, HasDenomType};
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::bootstrap::traits::types::genesis_config::HasGenesisConfigType;

pub struct DenomForStaking;

pub struct DenomForTransfer;

#[derive_component(GenesisDenomComponent, GenesisDenomGetter<Bootstrap>)]
pub trait HasGenesisDenom<Label>: HasGenesisConfigType + HasChainDriverType
where
    Self::ChainDriver: HasDenomType,
{
    fn genesis_denom(
        label: Label,
        chain_config: &Self::GenesisConfig,
    ) -> &DenomOf<Self::ChainDriver>;
}

#[derive_component(DenomPrefixGetterComponent, DenomPrefixGetter<Bootstrap>)]
pub trait HasDenomPrefix<Label>: Async {
    fn denom_prefix(&self, label: Label) -> &str;
}
