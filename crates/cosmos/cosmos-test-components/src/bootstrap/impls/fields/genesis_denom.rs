use cgp::prelude::*;
use hermes_test_components::chain::traits::types::denom::HasDenomType;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

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
    fn genesis_denom(_label: DenomForStaking, genesis_config: &CosmosGenesisConfig) -> &Denom {
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
    fn genesis_denom(_label: DenomForTransfer, genesis_config: &CosmosGenesisConfig) -> &Denom {
        &genesis_config.transfer_denom
    }
}
