use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::HasDenomType;
use hermes_test_components::chain_driver::traits::HasChainType;

use crate::bootstrap::traits::fields::denom::{
    DenomForStaking, DenomForTransfer, GenesisDenomGetter, GenesisDenomGetterComponent,
};
use crate::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use crate::bootstrap::types::genesis_config::CosmosGenesisConfig;
use crate::chain::types::denom::Denom;

pub struct GetCosmosGenesisDenoms;

#[cgp_provider(GenesisDenomGetterComponent)]
impl<Bootstrap, Chain> GenesisDenomGetter<Bootstrap, DenomForStaking> for GetCosmosGenesisDenoms
where
    Bootstrap: HasChainGenesisConfigType<ChainGenesisConfig = CosmosGenesisConfig>
        + HasChainType<Chain = Chain>,
    Chain: HasDenomType<Denom = Denom>,
{
    fn genesis_denom(
        genesis_config: &CosmosGenesisConfig,
        _label: PhantomData<DenomForStaking>,
    ) -> &Denom {
        &genesis_config.staking_denom
    }
}

#[cgp_provider(GenesisDenomGetterComponent)]
impl<Bootstrap, Chain> GenesisDenomGetter<Bootstrap, DenomForTransfer> for GetCosmosGenesisDenoms
where
    Bootstrap: HasChainGenesisConfigType<ChainGenesisConfig = CosmosGenesisConfig>
        + HasChainType<Chain = Chain>,
    Chain: HasDenomType<Denom = Denom>,
{
    fn genesis_denom(
        genesis_config: &CosmosGenesisConfig,
        _label: PhantomData<DenomForTransfer>,
    ) -> &Denom {
        &genesis_config.transfer_denom
    }
}
