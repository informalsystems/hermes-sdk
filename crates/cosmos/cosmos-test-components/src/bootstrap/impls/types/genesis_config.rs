use cgp::prelude::*;

use crate::bootstrap::components::cosmos_sdk::ChainGenesisConfigTypeComponent;
use crate::bootstrap::traits::types::genesis_config::ProvideChainGenesisConfigType;
use crate::bootstrap::types::genesis_config::CosmosGenesisConfig;

pub struct ProvideCosmosGenesisConfigType;

#[cgp_provider(ChainGenesisConfigTypeComponent)]
impl<Bootstrap> ProvideChainGenesisConfigType<Bootstrap> for ProvideCosmosGenesisConfigType
where
    Bootstrap: Async,
{
    type ChainGenesisConfig = CosmosGenesisConfig;
}
