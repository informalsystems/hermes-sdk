use cgp_core::prelude::*;
use hermes_test_components::bootstrap::traits::types::chain::HasChainType;
use hermes_test_components::chain::traits::types::denom::{Denom, HasDenomType};

use crate::bootstrap::traits::types::genesis_config::HasGenesisConfigType;

pub struct DenomForStaking;

pub struct DenomForTransfer;

#[derive_component(GenesisDenomComponent, GenesisDenomGetter<Bootstrap>)]
pub trait HasGenesisDenom<Label>: HasGenesisConfigType + HasChainType
where
    Self::Chain: HasDenomType,
{
    fn genesis_denom(chain_config: &Self::GenesisConfig) -> &Denom<Self::Chain>;
}
