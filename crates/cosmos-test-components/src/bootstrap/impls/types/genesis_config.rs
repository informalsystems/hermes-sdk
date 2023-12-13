use cgp_core::Async;

use crate::bootstrap::traits::types::genesis_config::ProvideGenesisConfigType;
use crate::bootstrap::types::genesis_config::CosmosGenesisConfig;

pub struct ProvideCosmosGenesisConfigType;

impl<Bootstrap> ProvideGenesisConfigType<Bootstrap> for ProvideCosmosGenesisConfigType
where
    Bootstrap: Async,
{
    type GenesisConfig = CosmosGenesisConfig;
}
