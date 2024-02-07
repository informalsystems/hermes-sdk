use cgp_core::prelude::*;
use hermes_test_components::chain_driver::traits::types::denom::{DenomOf, HasDenomType};
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

#[derive_component(GasDenomGetterComponent, GasDenomGetter<Bootstrap>)]
pub trait HasGasDenom: HasChainDriverType
where
    Self::ChainDriver: HasDenomType,
{
    fn gas_denom(&self) -> &DenomOf<Self::ChainDriver>;
}
