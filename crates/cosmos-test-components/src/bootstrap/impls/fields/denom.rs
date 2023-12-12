use cgp_core::prelude::*;
use ibc_test_components::bootstrap::traits::types::chain::HasChainType;
use ibc_test_components::chain::traits::types::denom::{Denom, HasDenomType};

pub struct DenomForStaking;

pub struct DenomForTransfer;

#[derive_component(GenesisDenomComponent, GenesisDenomGetter<Bootstrap>)]
pub trait HasGenesisDenom<Label>: HasChainType
where
    Self::Chain: HasDenomType,
{
    fn genesis_denom(&self) -> &Denom<Self::Chain>;
}
