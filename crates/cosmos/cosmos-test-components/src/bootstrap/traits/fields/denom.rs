use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_test_components::chain::traits::types::denom::{DenomOf, HasDenomType};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

use crate::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;

pub struct DenomForStaking;

pub struct DenomForTransfer;

#[derive_component(GenesisDenomGetterComponent, GenesisDenomGetter<Bootstrap>)]
pub trait HasGenesisDenom<Label>: HasChainGenesisConfigType + HasChainType
where
    Self::Chain: HasDenomType,
{
    fn genesis_denom(
        label: Label,
        chain_config: &Self::ChainGenesisConfig,
    ) -> &DenomOf<Self::Chain>;
}

#[derive_component(DenomPrefixGetterComponent, DenomPrefixGetter<Bootstrap>)]
pub trait HasDenomPrefix<Label>: Async {
    fn denom_prefix(&self, label: Label) -> &str;
}

impl<Bootstrap> DenomPrefixGetter<Bootstrap, DenomForStaking> for UseContext
where
    Bootstrap: Async + HasField<symbol!("staking_denom_prefix"), Value = String>,
{
    fn denom_prefix(bootstrap: &Bootstrap, _label: DenomForStaking) -> &str {
        bootstrap.get_field(PhantomData)
    }
}

impl<Bootstrap> DenomPrefixGetter<Bootstrap, DenomForTransfer> for UseContext
where
    Bootstrap: Async + HasField<symbol!("transfer_denom_prefix"), Value = String>,
{
    fn denom_prefix(bootstrap: &Bootstrap, _label: DenomForTransfer) -> &str {
        bootstrap.get_field(PhantomData)
    }
}
