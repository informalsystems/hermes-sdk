use cgp_core::Async;

use crate::bootstrap::traits::types::genesis_config::ProvideChainGenesisConfigType;
use crate::bootstrap::types::genesis_config::CosmosGenesisConfig;

pub struct ProvideCosmosGenesisConfigType;

impl<Bootstrap> ProvideChainGenesisConfigType<Bootstrap> for ProvideCosmosGenesisConfigType
where
    Bootstrap: Async,
{
    type ChainGenesisConfig = CosmosGenesisConfig;
}
