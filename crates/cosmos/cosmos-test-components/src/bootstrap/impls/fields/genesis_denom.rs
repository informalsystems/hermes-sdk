use hermes_test_components::chain_driver::traits::types::denom::HasDenomType;
use hermes_test_components::driver::traits::types::chain_driver::HasChainDriverType;

use crate::bootstrap::traits::fields::denom::{
    DenomForStaking, DenomForTransfer, GenesisDenomGetter,
};
use crate::bootstrap::traits::types::genesis_config::HasChainGenesisConfigType;
use crate::bootstrap::types::genesis_config::CosmosGenesisConfig;
use crate::chain_driver::types::denom::Denom;

pub struct GetCosmosGenesisDenoms;

impl<Bootstrap, ChainDriver> GenesisDenomGetter<Bootstrap, DenomForStaking>
    for GetCosmosGenesisDenoms
where
    Bootstrap: HasChainGenesisConfigType<ChainGenesisConfig = CosmosGenesisConfig>
        + HasChainDriverType<ChainDriver = ChainDriver>,
    ChainDriver: HasDenomType<Denom = Denom>,
{
    fn genesis_denom(_label: DenomForStaking, genesis_config: &CosmosGenesisConfig) -> &Denom {
        &genesis_config.staking_denom
    }
}

impl<Bootstrap, ChainDriver> GenesisDenomGetter<Bootstrap, DenomForTransfer>
    for GetCosmosGenesisDenoms
where
    Bootstrap: HasChainGenesisConfigType<ChainGenesisConfig = CosmosGenesisConfig>
        + HasChainDriverType<ChainDriver = ChainDriver>,
    ChainDriver: HasDenomType<Denom = Denom>,
{
    fn genesis_denom(_label: DenomForTransfer, genesis_config: &CosmosGenesisConfig) -> &Denom {
        &genesis_config.transfer_denom
    }
}
